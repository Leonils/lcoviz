use lcov_aggregator_report::{
    adapters::{
        exporters::{mpa::MpaExporter, mpa_links::MpaLinksComputer},
        renderers::html_light_renderer::HtmlLightRenderer,
    },
    aggregation::{input::AggregatorInput, multi_report::MultiReport, tested_root::TestedRoot},
    cli::{
        config::Config,
        parser::{CliCommand, CliConfigParser},
    },
    core::{Exporter, LocalFileSystem},
};
use std::{collections::HashMap, env::args, error::Error, path::PathBuf};

fn handle_multi_report(config: Config) -> Result<MultiReport, Box<dyn Error>> {
    let mut report_names = HashMap::<String, u32>::new();
    let mut report_inputs = Vec::<AggregatorInput>::new();

    for config_input in config.inputs.into_iter() {
        let aggregator_input = AggregatorInput::from_config_input(config_input);
        let wanted_key = aggregator_input.last_part_of_prefix().to_string();
        report_names
            .entry(wanted_key)
            .and_modify(|e| *e += 1)
            .or_insert(1);
        report_inputs.push(aggregator_input);
    }

    let mut multi_report = MultiReport::new(&config.name);
    let mut dedup_counters = HashMap::<String, u32>::new();
    for mut input in report_inputs {
        let key = input.last_part_of_prefix().to_string();
        let count = report_names.get(&key).unwrap().to_owned();
        let key = if count > 1 {
            let c = dedup_counters
                .entry(key.clone())
                .and_modify(|e| *e += 1)
                .or_insert(1);
            format!("{}_{}", key, c)
        } else {
            key
        };
        input = input.with_key(&key);
        multi_report.add_report(TestedRoot::new(input));
    }

    Ok(multi_report)
}

fn handle_single_report(config: Config) -> Result<TestedRoot, Box<dyn Error>> {
    let input = config.inputs.into_iter().next().unwrap();
    let aggregator_input = AggregatorInput::from_config_input(input).with_name(&config.name);
    let tested_root = TestedRoot::new(aggregator_input);
    Ok(tested_root)
}

fn print_status(title: &str, status: &str) {
    const BOLD: &str = "\x1b[1m";
    const GREEN: &str = "\x1b[32m";
    const RESET: &str = "\x1b[0m";
    println!("{}{}{: >12} {}{}", BOLD, GREEN, title, RESET, status);
}

fn run_report(config: Config) -> Result<(), Box<dyn Error>> {
    let links_computer = MpaLinksComputer;
    let file_system = LocalFileSystem;
    let renderer = HtmlLightRenderer::new(links_computer);

    print_status(
        "Generating",
        &format!(
            "HTML report for {} input(s) lcov files",
            config.inputs.len()
        ),
    );

    print_status("", &format!("Report name: '{}'", config.name));
    print_status("", "Inputs: ");
    for input in config.inputs.iter() {
        print_status(
            "",
            &format!(
                "  - {}{}",
                input
                    .name
                    .as_ref()
                    .map_or("".to_string(), |f| format!("{}: ", f)),
                input.path.display()
            ),
        );
    }

    let output = config.output.clone();
    if config.inputs.len() != 1 {
        let multi_report = handle_multi_report(config)?;
        let exporter = MpaExporter::new(renderer, multi_report, &output, &file_system);
        exporter.render_root();
    } else {
        let root = handle_single_report(config)?;
        let exporter = MpaExporter::new(renderer, root, &output, &file_system);
        exporter.render_root();
    };

    print_status(
        "Success",
        format!("Report generated at {}", output.display()).as_str(),
    );

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

fn main() -> Result<(), Box<dyn Error>> {
    let args = args().skip(1).collect::<Vec<String>>();
    let command = CliConfigParser::new().parse(&args)?.build()?;

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
