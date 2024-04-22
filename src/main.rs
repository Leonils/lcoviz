use lcov::Report;
use lcov_aggregator_report::{
    adapters::{
        exporters::{mpa::MpaExporter, mpa_links::MpaLinksComputer},
        renderers::html_light_renderer::HtmlLightRenderer,
    },
    aggregation::{input::AggregatorInput, tested_root::TestedRoot},
    core::{Exporter, LocalFileSystem},
};
use std::{env::args, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = args();

    let _ = args.next();
    let input_path = args.next().map(PathBuf::from).expect("Missing input path");
    let output_path = args.next().map(PathBuf::from).expect("Missing output path");

    let report = Report::from_file(input_path)?;
    let aggregator_input = AggregatorInput::new(report).with_longest_prefix();
    let tested_root = TestedRoot::new(aggregator_input);
    let links_computer = MpaLinksComputer::new(&LocalFileSystem);
    let renderer = HtmlLightRenderer::new(links_computer);

    let file_system = LocalFileSystem;
    let exporter = MpaExporter::new(renderer, tested_root, output_path, &file_system);
    exporter.render_root();

    Ok(())
}
