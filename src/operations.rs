use crate::{
    adapters::{
        cli::{
            cli_output::{CliOutput, Console},
            parser::{CliCommand, CliConfigParser},
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
use std::path::PathBuf;

fn build_multi_report_root(config: Config) -> Result<MultiReport, String> {
    let mut multi_report = MultiReport::new(&config.name);
    for input in AggregatorInput::build_from_inputs(config.inputs, &LocalFileSystem) {
        multi_report.add_report(TestedRoot::new(input));
    }
    Ok(multi_report)
}

fn build_single_report_root(config: Config) -> Result<TestedRoot, String> {
    let input = config.inputs.into_iter().next().unwrap();
    let aggregator_input =
        AggregatorInput::from_config_input(input, &LocalFileSystem).with_name(&config.name);
    let tested_root = TestedRoot::new(aggregator_input);
    Ok(tested_root)
}

macro_rules! export {
    ($exporter_struct: ident, $renderer: expr, $config: expr) => {{
        let output = $config.output.clone();
        if $config.inputs.len() != 1 {
            let multi_report = build_multi_report_root($config)?;
            $exporter_struct::new($renderer, multi_report, &output, &LocalFileSystem).render_root();
            Ok::<(), String>(())
        } else {
            let root = build_single_report_root($config)?;
            $exporter_struct::new($renderer, root, &output, &LocalFileSystem).render_root();
            Ok::<(), String>(())
        }
    }};
}

fn run_report(config: Config, cli_output: &CliOutput<Console>) -> Result<(), String> {
    let output = config.output.clone();

    cli_output.print_introduction(&config);

    match config.reporter {
        Reporter::MpaHtmlLightReporter => export!(
            MpaExporter,
            HtmlLightRenderer::new(MpaLinksComputer),
            config
        )?,
        Reporter::TextSummaryReporter => export!(SpaExporter, TextSinglePageRenderer, config)?,
    };

    cli_output.print_conclusion(&output.display().to_string());
    Ok(())
}

fn save_config_to_file(config: Config, path: &PathBuf) -> Result<(), String> {
    let config_str = toml::to_string(&config).map_err(|e| e.to_string())?;
    if path.exists() {
        println!("File already exists, overwriting it?");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .map_err(|e| e.to_string())?;
        if input.to_lowercase().trim() != "y" {
            println!("Aborting");
        }
    }
    if path.parent().is_some() {
        std::fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;
    }
    std::fs::write(path, config_str).map_err(|e| e.to_string())?;
    Ok(())
}

fn read_config_from_file(path: &PathBuf) -> Result<Config, String> {
    let config_str = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let config = toml::from_str::<Config>(&config_str).map_err(|e| e.to_string())?;
    Ok(config)
}

fn run_command(args: Vec<String>, cli_output: &CliOutput<Console>) -> Result<(), String> {
    let command = CliConfigParser::new().parse(&args)?.build()?;
    match command {
        CliCommand::Report(config) => run_report(config, &cli_output)?,
        CliCommand::ToFile(path, config) => save_config_to_file(config, &path)?,
        CliCommand::FromFile(path) => {
            let config = read_config_from_file(&path)?;
            run_report(config, &cli_output)?
        }
    };
    Ok(())
}

pub fn run(args: Vec<String>) -> () {
    let cli_output = CliOutput::new(Console);
    match run_command(args, &cli_output) {
        Ok(_) => (),
        Err(e) => cli_output.print_error(&e),
    }
}
