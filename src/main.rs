use lcov::Report;
use lcov_aggregator_report::adapters::renderers::html_light_renderer::HtmlLightRenderer;
use lcov_aggregator_report::aggregation::input::AggregatorInput;
use lcov_aggregator_report::aggregation::tested_root::TestedRoot;
use lcov_aggregator_report::core::{Renderer, WithPath};
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
    let renderer = HtmlLightRenderer {
        root: Box::new(tested_root),
    };

    // overview
    std::fs::create_dir_all(&output_path)?;
    let mut file =
        std::fs::File::create(output_path.join("index.html")).expect("Failed to create index.html");
    file.write_all(renderer.render_coverage_summary().as_bytes())?;

    // details
    for file in renderer.root.enumerate_code_files() {
        println!("Rendering file: {}", file.get_path_string());
        let lines_provider = LocalFileLinesProvider::new(file.get_path());
        let mut target_path = output_path
            .join("details")
            .join(file.get_path_relative_to(&renderer.root.get_path()));

        let extension = target_path.extension().unwrap_or_default();
        target_path.set_extension(format!("{}.html", extension.to_string_lossy()));

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
