use lcov::Report;
use lcov_aggregator_report::{
    file_provider::LocalFileLinesProvider, models::to_html::ToHtmlWithLinesProvider,
    styles::light::MockComponentsFactory,
};
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

    let mut i = 0;
    for (section_key, section) in &report.sections {
        println!("\nSection: {:?}", section_key.source_file);
        println!("Test: {:?}", section_key.test_name);

        let components = MockComponentsFactory {};
        let lines_provider = LocalFileLinesProvider::new(section_key.source_file.clone());
        let html = section.lines.to_html(components, lines_provider);

        // write to file
        let mut file = std::fs::File::create(format!(
            "{}/report_{}.html",
            output_path.to_str().unwrap(),
            i
        ))?;
        file.write_all(html.render().as_bytes())?;
        i += 1;
        println!("Saved to file report_{}.html", i);
    }

    Ok(())
}
