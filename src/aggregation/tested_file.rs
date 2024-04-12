use crate::core::{AggregatedCoverage, TestedFile};

use super::with_path::WithPath;
use lcov::report::section::{Key as SectionKey, Value as SectionValue};

#[derive(Debug, PartialEq, Default)]
pub struct TestedCodeFile {
    file_name: String,
    path: String,
    aggregated: AggregatedCoverage,
}

impl TestedCodeFile {
    pub fn new(path: &str, file_name: &str) -> Self {
        TestedCodeFile {
            file_name: String::from(file_name),
            path: String::from(path),
            aggregated: AggregatedCoverage::default(),
        }
    }

    pub fn from_section(key: SectionKey, value: SectionValue) -> Self {
        let path = key.source_file.to_str().unwrap().to_string();
        let file_name = path.split('/').last().unwrap().to_string();
        let aggregated = AggregatedCoverage::from_section(value);

        TestedCodeFile {
            file_name,
            path,
            aggregated,
        }
    }
}

impl TestedFile for TestedCodeFile {
    fn get_file_path(&self) -> &str {
        &self.file_name
    }

    fn get_aggregated_coverage(&self) -> &AggregatedCoverage {
        &self.aggregated
    }
}

impl WithPath for TestedCodeFile {
    fn get_path_string(&self) -> String {
        self.path.clone()
    }
}

#[cfg(test)]
impl TestedCodeFile {
    pub fn with_aggregated(path: &str, file_name: &str, aggregated: AggregatedCoverage) -> Self {
        TestedCodeFile {
            aggregated,
            ..TestedCodeFile::new(path, file_name)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        aggregation::aggregated::assert_aggregated_counters_eq,
        test_utils::builders::generate_3_lines_2_covered_section,
    };

    use super::*;

    #[test]
    fn when_creating_a_tested_file_the_aggregated_count_should_be_0() {
        let tested_file = TestedCodeFile::new("path", "file");
        assert_eq!(tested_file.aggregated, AggregatedCoverage::default());
    }

    #[test]
    fn when_getting_the_path_string_it_should_return_the_path() {
        let tested_file = TestedCodeFile::new("path", "file");
        assert_eq!(tested_file.get_path_string(), "path");
    }

    #[test]
    fn when_creating_from_a_section_it_shall_get_its_name_from_it() {
        let key = SectionKey {
            source_file: std::path::PathBuf::from("path/file.cpp"),
            test_name: String::from(""),
        };
        let value = SectionValue::default();

        let tested_file = TestedCodeFile::from_section(key, value);
        assert_eq!(tested_file.file_name, "file.cpp");
    }

    #[test]
    fn when_creating_from_an_empty_section_aggregate_shall_be_0() {
        let key = SectionKey {
            source_file: std::path::PathBuf::from("path/file.cpp"),
            test_name: String::from(""),
        };
        let value = SectionValue::default();

        let tested_file = TestedCodeFile::from_section(key, value);
        assert_aggregated_counters_eq(&tested_file.aggregated.lines, 0, 0);
    }

    #[test]
    fn when_creating_from_a_sections_with_lines_aggregate_shall_count_covered_lines() {
        let key = SectionKey {
            source_file: std::path::PathBuf::from("path/file.cpp"),
            test_name: String::from(""),
        };

        let section_value = generate_3_lines_2_covered_section();

        let tested_file = TestedCodeFile::from_section(key, section_value);
        assert_aggregated_counters_eq(&tested_file.aggregated.lines, 3, 2);
    }
}
