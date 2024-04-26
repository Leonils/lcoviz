use crate::{
    adapters::{
        cli::{
            cli_output::{CliOutput, Console},
            parser::CliCommand,
        },
        exporters::{mpa::MpaExporter, mpa_links::MpaLinksComputer, spa::SpaExporter},
        renderers::{
            html_light_renderer::HtmlLightRenderer,
            text_single_page_renderer::TextSinglePageRenderer,
        },
    },
    aggregation::{multi_report::MultiReport, tested_root::TestedRoot},
    core::{Exporter, LocalFileSystem},
    input::{
        aggregator_input::AggregatorInput,
        config::{Config, Reporter},
    },
};
use std::{error::Error, path::PathBuf};

fn build_multi_report_root(config: Config) -> Result<MultiReport, Box<dyn Error>> {
    let mut multi_report = MultiReport::new(&config.name);
    for input in AggregatorInput::build_from_inputs(config.inputs, &LocalFileSystem) {
        multi_report.add_report(TestedRoot::new(input));
    }
    Ok(multi_report)
}

fn build_single_report_root(config: Config) -> Result<TestedRoot, Box<dyn Error>> {
    let input = config.inputs.into_iter().next().unwrap();
    let aggregator_input =
        AggregatorInput::from_config_input(input, &LocalFileSystem).with_name(&config.name);
    let tested_root = TestedRoot::new(aggregator_input);
    Ok(tested_root)
}

fn export_mpa_html_light_renderer(config: Config) -> Result<(), Box<dyn Error>> {
    let output = config.output.clone();
    let links_computer = MpaLinksComputer;
    let renderer = HtmlLightRenderer::new(links_computer);
    if config.inputs.len() != 1 {
        let multi_report = build_multi_report_root(config)?;
        MpaExporter::new(renderer, multi_report, &output, &LocalFileSystem).render_root();
    } else {
        let root = build_single_report_root(config)?;
        MpaExporter::new(renderer, root, &output, &LocalFileSystem).render_root();
    };
    Ok(())
}

fn export_text_summary(config: Config) -> Result<(), Box<dyn Error>> {
    let output = config.output.clone();
    let renderer = TextSinglePageRenderer;
    if config.inputs.len() != 1 {
        let multi_report = build_multi_report_root(config)?;
        SpaExporter::new(renderer, multi_report, &output, &LocalFileSystem).render_root();
    } else {
        let root = build_single_report_root(config)?;
        SpaExporter::new(renderer, root, &output, &LocalFileSystem).render_root();
    };
    Ok(())
}

fn run_report(config: Config) -> Result<(), Box<dyn Error>> {
    let output = config.output.clone();
    let cli_output = CliOutput::new(Console);

    cli_output.print_introduction(&config);

    match config.reporter {
        Reporter::MpaHtmlLightReporter => export_mpa_html_light_renderer(config)?,
        Reporter::TextSummaryReporter => export_text_summary(config)?,
    };

    cli_output.print_conclusion(&output.display().to_string());
    Ok(())
}

fn save_config_to_file(config: Config, path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let config_str = toml::to_string(&config)?;
    if path.exists() {
        println!("File already exists, overwriting it?");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        if input.to_lowercase().trim() != "y" {
            println!("Aborting");
        }
    }
    if path.parent().is_some() {
        std::fs::create_dir_all(path.parent().unwrap())?;
    }
    std::fs::write(path, config_str)?;
    Ok(())
}

fn read_config_from_file(path: &PathBuf) -> Result<Config, Box<dyn Error>> {
    let config_str = std::fs::read_to_string(path)?;
    let config = toml::from_str::<Config>(&config_str)?;
    Ok(config)
}

pub fn run(command: CliCommand) -> Result<(), Box<dyn Error>> {
    match command {
        CliCommand::Report(config) => run_report(config)?,
        CliCommand::ToFile(path, config) => save_config_to_file(config, &path)?,
        CliCommand::FromFile(path) => {
            let config = read_config_from_file(&path)?;
            run_report(config)?;
        }
    }

    Ok(())
}
