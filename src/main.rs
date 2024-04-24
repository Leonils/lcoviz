use lcov::Report;
use lcov_aggregator_report::{
    adapters::{
        exporters::{mpa::MpaExporter, mpa_links::MpaLinksComputer},
        renderers::html_light_renderer::HtmlLightRenderer,
    },
    aggregation::{input::AggregatorInput, multi_report::MultiReport, tested_root::TestedRoot},
    core::{Exporter, LocalFileSystem},
};
use std::{collections::HashMap, env::args, error::Error, path::PathBuf};

fn handle_multi_report(input_paths: &[PathBuf]) -> Result<MultiReport, Box<dyn Error>> {
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

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = args();

    let _ = args.next();

    // first n-1 arguments are the input paths, the last one is the output path
    let inputs = args.map(PathBuf::from).collect::<Vec<_>>();
    let input_paths = &inputs[..inputs.len() - 1];
    let output_path = inputs.last().expect("Missing output path").to_owned();
    if input_paths.is_empty() {
        panic!("Missing input path");
    }

    let links_computer = MpaLinksComputer;
    let file_system = LocalFileSystem;
    let renderer = HtmlLightRenderer::new(links_computer);

    if input_paths.len() != 1 {
        let multi_report = handle_multi_report(input_paths)?;
        let exporter = MpaExporter::new(renderer, multi_report, output_path, &file_system);
        exporter.render_root();
    } else {
        let root = handle_single_report(input_paths.first().unwrap())?;
        let exporter = MpaExporter::new(renderer, root, output_path, &file_system);
        exporter.render_root();
    };

    Ok(())
}
