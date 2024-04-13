use lcov::Report;
use lcov_aggregator_report::adapters::renderers::html_light_renderer::HtmlLightRenderer;
use lcov_aggregator_report::aggregation::input::AggregatorInput;
use lcov_aggregator_report::aggregation::tested_root::TestedRoot;
use lcov_aggregator_report::core::{Renderer, TestedFile};
use lcov_aggregator_report::file_provider::LocalFileLinesProvider;
use std::env::args;
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = args();

    let _ = args.next();
    let input_path = args.next().map(PathBuf::from).expect("Missing input path");
    let output_path = args.next().map(PathBuf::from).expect("Missing output path");

    let report = Report::from_file(input_path)?;
    let aggregator_input = AggregatorInput::new(report).with_longest_prefix();
    let tested_root = TestedRoot::new(aggregator_input);
    let renderer = HtmlLightRenderer {};

    // overview
    std::fs::create_dir_all(&output_path)?;
    let mut file =
        std::fs::File::create(output_path.join("index.html")).expect("Failed to create index.html");
    file.write_all(renderer.render_coverage_summary(&tested_root).as_bytes())?;

    // details
    for file in tested_root.enumerate_code_files() {
        println!("Rendering file: {}", file.get_file_path());
        let lines_provider = LocalFileLinesProvider::new(PathBuf::from(file.get_file_path()));
        let mut target_path = output_path
            .join("details")
            .join(file.get_path_relative_to_prefix());
        target_path.set_extension("html");

        std::fs::create_dir_all(target_path.parent().unwrap())?;

        let mut f = std::fs::File::create(target_path)?;
        f.write_all(
            renderer
                .render_file_coverage_details(file, lines_provider)
                .as_bytes(),
        )?;
    }

    Ok(())
}
