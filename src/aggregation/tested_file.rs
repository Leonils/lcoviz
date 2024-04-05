use super::with_path::WithPath;
use crate::aggregation::aggregated::Aggregated;
use lcov::report::section::{Key as SectionKey, Value as SectionValue};

#[derive(Debug, PartialEq, Default)]
pub struct TestedFile {
    file_name: String,
    path: String,
    pub aggregated: Aggregated,
}

impl TestedFile {
    pub fn new(path: &str, file_name: &str) -> Self {
        TestedFile {
            file_name: String::from(file_name),
            path: String::from(path),
            aggregated: Aggregated::default(),
        }
    }

    pub fn from_section(key: SectionKey, value: SectionValue) -> Self {
        let path = key.source_file.to_str().unwrap().to_string();
        let file_name = path.split('/').last().unwrap().to_string();
        let aggregated = Aggregated::from_section(value);

        TestedFile {
            file_name,
            path,
            aggregated,
        }
    }
}

impl WithPath for TestedFile {
    fn get_path_string(&self) -> String {
        self.path.clone()
    }
}

#[cfg(test)]
impl TestedFile {
    pub fn with_aggregated(path: &str, file_name: &str, aggregated: Aggregated) -> Self {
        TestedFile {
            aggregated,
            ..TestedFile::new(path, file_name)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{aggregation::aggregated::Aggregated, test_utils::builders::InsertLine};

    use super::*;

    #[test]
    fn when_creating_a_tested_file_the_aggregated_count_should_be_0() {
        let tested_file = TestedFile::new("path", "file");
        assert_eq!(tested_file.aggregated, Aggregated::default());
    }

    #[test]
    fn when_getting_the_path_string_it_should_return_the_path() {
        let tested_file = TestedFile::new("path", "file");
        assert_eq!(tested_file.get_path_string(), "path");
    }

    #[test]
    fn when_creating_from_a_section_it_shall_get_its_name_from_it() {
        let key = SectionKey {
            source_file: std::path::PathBuf::from("path/file.cpp"),
            test_name: String::from(""),
        };
        let value = SectionValue::default();

        let tested_file = TestedFile::from_section(key, value);
        assert_eq!(tested_file.file_name, "file.cpp");
    }

    #[test]
    fn when_creating_from_an_empty_section_aggregate_shall_be_0() {
        let key = SectionKey {
            source_file: std::path::PathBuf::from("path/file.cpp"),
            test_name: String::from(""),
        };
        let value = SectionValue::default();

        let tested_file = TestedFile::from_section(key, value);
        assert_eq!(tested_file.aggregated.lines_count, 0);
        assert_eq!(tested_file.aggregated.covered_lines_count, 0);
    }

    #[test]
    fn when_creating_from_a_sections_with_lines_aggregate_shall_count_covered_lines() {
        let key = SectionKey {
            source_file: std::path::PathBuf::from("path/file.cpp"),
            test_name: String::from(""),
        };

        let mut section_value = SectionValue::default();
        section_value
            .lines
            .insert_line(1, 0)
            .insert_line(2, 3)
            .insert_line(3, 1);

        let tested_file = TestedFile::from_section(key, section_value);
        assert_eq!(tested_file.aggregated.lines_count, 3);
        assert_eq!(tested_file.aggregated.covered_lines_count, 2);
    }
}
