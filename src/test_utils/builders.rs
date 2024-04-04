use std::path::PathBuf;

pub trait FromStr {
    fn from_str(text: &str) -> Self;
}
pub trait FromLineNumber {
    fn from_line_number(line_number: u32) -> Self;
}

pub trait FromCount {
    fn from_count(count: u64) -> Self;
}

impl FromStr for lcov::report::section::Key {
    fn from_str(text: &str) -> Self {
        lcov::report::section::Key {
            source_file: PathBuf::from(text),
            test_name: String::from(""),
        }
    }
}

impl FromLineNumber for lcov::report::section::line::Key {
    fn from_line_number(line_number: u32) -> Self {
        lcov::report::section::line::Key { line: line_number }
    }
}

impl FromCount for lcov::report::section::line::Value {
    fn from_count(count: u64) -> Self {
        lcov::report::section::line::Value {
            count,
            ..Default::default()
        }
    }
}
