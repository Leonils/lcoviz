use std::path::PathBuf;

pub trait FromStr {
    fn from_str(text: &str) -> Self;
}

impl FromStr for lcov::report::section::Key {
    fn from_str(text: &str) -> Self {
        lcov::report::section::Key {
            source_file: PathBuf::from(text),
            test_name: String::from(""),
        }
    }
}
