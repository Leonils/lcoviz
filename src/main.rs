use lcov_aggregator_report::{adapters::cli::parser::CliConfigParser, operations::run};
use std::{env::args, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args = args().skip(1).collect::<Vec<String>>();
    let command = CliConfigParser::new().parse(&args)?.build()?;
    run(command)
}
