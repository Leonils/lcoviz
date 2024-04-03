use std::error::Error;

use lcov::Report;

fn main() -> Result<(), Box<dyn Error>> {
    let report = Report::from_file("tests/fixtures/report.info")?;

    Ok(())
}
