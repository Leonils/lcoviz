use lcov::Report;
use lcov_aggregator_report::{
    adapters::{
        exporters::{mpa::MpaExporter, mpa_links::MpaLinksComputer},
        renderers::html_light_renderer::HtmlLightRenderer,
    },
    aggregation::{input::AggregatorInput, multi_report::MultiReport, tested_root::TestedRoot},
    cli::parser::CliConfigParser,
    core::{Exporter, LocalFileSystem},
};
use std::{collections::HashMap, env::args, error::Error, path::PathBuf};

fn handle_multi_report(input_paths: Vec<PathBuf>) -> Result<MultiReport, Box<dyn Error>> {
    let mut report_names = HashMap::<String, u32>::new();
    let mut report_inputs = Vec::<AggregatorInput>::new();
    for input_path in input_paths {
        let other_report = Report::from_file(input_path)?;
        let aggregator_input = AggregatorInput::new(other_report).with_longest_prefix();
        let wanted_key = aggregator_input.last_part_of_prefix().to_string();
        report_names
            .entry(wanted_key)
            .and_modify(|e| *e += 1)
            .or_insert(1);
        report_inputs.push(aggregator_input);
    }

    let mut multi_report = MultiReport::new();
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

fn handle_single_report(input_path: &PathBuf) -> Result<TestedRoot, Box<dyn Error>> {
    let input = Report::from_file(input_path)?;
    let aggregator_input = AggregatorInput::new(input).with_longest_prefix();
    let tested_root = TestedRoot::new(aggregator_input);
    Ok(tested_root)
}

fn print_status(title: &str, status: &str) {
    const BOLD: &str = "\x1b[1m";
    const GREEN: &str = "\x1b[32m";
    const RESET: &str = "\x1b[0m";
    println!("  {}{}{}{}: {}", BOLD, GREEN, title, RESET, status);
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = args().skip(1).collect::<Vec<String>>();
    let config = CliConfigParser::new().parse(&args)?.build()?;

    let links_computer = MpaLinksComputer;
    let file_system = LocalFileSystem;
    let renderer = HtmlLightRenderer::new(links_computer);
    let input_paths = config
        .inputs
        .iter()
        .map(|p| p.get_path())
        .collect::<Vec<PathBuf>>();

    print_status(
        "Generating report",
        format!("for {} input(s)", input_paths.len()).as_str(),
    );

    if config.inputs.len() != 1 {
        let multi_report = handle_multi_report(input_paths)?;
        let exporter = MpaExporter::new(renderer, multi_report, &config.output, &file_system);
        exporter.render_root();
    } else {
        let root = handle_single_report(input_paths.first().unwrap())?;
        let exporter = MpaExporter::new(renderer, root, &config.output, &file_system);
        exporter.render_root();
    };

    print_status(
        "Success",
        format!("Report generated at {}", config.output.display()).as_str(),
    );

    Ok(())
}
