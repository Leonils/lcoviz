use std::path::PathBuf;

use crate::core::{AggregatedCoverage, TestedFile, WithPath};

use lcov::report::section::line::Key as LineKey;
use lcov::report::section::{Key as SectionKey, Value as SectionValue};

#[derive(Debug, PartialEq, Default)]
pub struct TestedCodeFile {
    file_name: String,
    path: String,
    path_relative_to_prefix: String,
    aggregated: AggregatedCoverage,
    section: SectionValue,
}

impl TestedCodeFile {
    #[cfg(test)]
    pub fn new(path: &str, file_name: &str) -> Self {
        TestedCodeFile {
            file_name: String::from(file_name),
            path: String::from(path),
            path_relative_to_prefix: String::from(path),
            aggregated: AggregatedCoverage::default(),
            section: SectionValue::default(),
        }
    }

    pub fn from_section(
        key: SectionKey,
        value: SectionValue,
        prefix: &str,
        report_key: &str,
    ) -> Self {
        let path = key.source_file.to_str().unwrap().to_string();
        let file_name = path.split('/').last().unwrap().to_string();
        let aggregated = AggregatedCoverage::from_section(&value);
        let binding = PathBuf::from(prefix);
        let prefix_parts = binding.components().collect::<Vec<_>>();

        let source_file_parts = key.source_file.components().skip(prefix_parts.len());
        let path_relative_to_prefix = PathBuf::from(report_key)
            .join(PathBuf::from_iter(source_file_parts))
            .to_str()
            .unwrap()
            .to_string();

        TestedCodeFile {
            file_name,
            path,
            aggregated,
            section: value,
            path_relative_to_prefix,
        }
    }
}

impl TestedFile for TestedCodeFile {
    fn get_original_file_path(&self) -> PathBuf {
        PathBuf::from(&self.path)
    }

    fn get_aggregated_coverage(&self) -> &AggregatedCoverage {
        &self.aggregated
    }

    fn get_line_coverage(&self, line: u32) -> Option<u64> {
        self.section
            .lines
            .get(&LineKey { line })
            .map_or(None, |value| Some(value.count))
    }

    fn get_functions(&self) -> impl Iterator<Item = (String, u64)> {
        self.section
            .functions
            .iter()
            .map(|(key, value)| (key.name.clone(), value.count))
    }
}

impl WithPath for TestedCodeFile {
    fn get_name(&self) -> &str {
        &self.file_name
    }

    fn is_dir(&self) -> bool {
        false
    }

    fn get_path_string(&self) -> String {
        self.path_relative_to_prefix.clone()
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

        let tested_file = TestedCodeFile::from_section(key, value, "", "");
        assert_eq!(tested_file.file_name, "file.cpp");
    }

    #[test]
    fn when_creating_from_a_section_with_a_prefix_it_shall_get_the_path_relative_to_prefix() {
        let key = SectionKey {
            source_file: std::path::PathBuf::from("path/file.cpp"),
            test_name: String::from(""),
        };
        let value = SectionValue::default();

        let tested_file = TestedCodeFile::from_section(key, value, "path/", "");
        assert_eq!(tested_file.path_relative_to_prefix, "file.cpp");
    }

    #[test]
    fn when_creating_from_an_empty_section_aggregate_shall_be_0() {
        let key = SectionKey {
            source_file: std::path::PathBuf::from("path/file.cpp"),
            test_name: String::from(""),
        };
        let value = SectionValue::default();

        let tested_file = TestedCodeFile::from_section(key, value, "", "");
        assert_aggregated_counters_eq(&tested_file.aggregated.lines, 0, 0);
    }

    #[test]
    fn when_creating_from_a_sections_with_lines_aggregate_shall_count_covered_lines() {
        let key = SectionKey {
            source_file: std::path::PathBuf::from("path/file.cpp"),
            test_name: String::from(""),
        };

        let section_value = generate_3_lines_2_covered_section();

        let tested_file = TestedCodeFile::from_section(key, section_value, "", "");
        assert_aggregated_counters_eq(&tested_file.aggregated.lines, 3, 2);
    }
}
