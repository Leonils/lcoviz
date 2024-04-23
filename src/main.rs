use lcov::Report;
use lcov_aggregator_report::{
    adapters::{
        exporters::{mpa::MpaExporter, mpa_links::MpaLinksComputer},
        renderers::html_light_renderer::HtmlLightRenderer,
    },
    aggregation::{input::AggregatorInput, tested_root::TestedRoot},
    core::{Exporter, LocalFileSystem, WithPath},
};
use std::{env::args, error::Error, path::PathBuf};

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

    // let input_path = args.next().map(PathBuf::from).expect("Missing input path");
    // let output_path = args.next().map(PathBuf::from).expect("Missing output path");

    let mut report = Report::new();
    for input_path in input_paths {
        let other_report = Report::from_file(input_path)?;
        report.merge(other_report)?;
    }

    let aggregator_input = AggregatorInput::new(report).with_longest_prefix();
    let tested_root = TestedRoot::new(aggregator_input);
    if tested_root.get_path_string().is_empty() {
        panic!("Cannot find common prefix in the input paths");
    }

    let links_computer = MpaLinksComputer::new(&LocalFileSystem);
    let renderer = HtmlLightRenderer::new(links_computer);

    let file_system = LocalFileSystem;
    let exporter = MpaExporter::new(renderer, tested_root, output_path, &file_system);
    exporter.render_root();

    Ok(())
}
