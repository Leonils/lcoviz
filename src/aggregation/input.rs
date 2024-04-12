use lcov::report::section::{Key as SectionKey, Value as SectionValue};
use std::collections::BTreeMap;

pub struct AggregatorInput {
    report: lcov::report::Report,
}

impl AggregatorInput {
    pub fn new(report: lcov::report::Report) -> Self {
        Self { report }
    }

    pub fn list_sections(&self) -> BTreeMap<SectionKey, SectionValue> {
        self.report.sections.clone()
    }

    pub fn with_prefix(&self, prefix: &str) -> AggregatorInput {
        let mut report = self.report.clone();
        let sections = report.sections.clone();
        let new_sections = sections
            .into_iter()
            .map(|(key, value)| {
                let mut new_key = key.clone();

                // remove prefix from the source file
                let source_file = new_key.source_file.to_str().unwrap();
                let new_source_file = source_file.trim_start_matches(prefix);
                new_key.source_file = std::path::PathBuf::from(new_source_file);

                (new_key, value)
            })
            .collect();
        report.sections = new_sections;
        AggregatorInput::new(report)
    }

    fn drop_file_name(source_file: &str) -> Vec<&str> {
        let parts = source_file.split('/').collect::<Vec<&str>>();
        let len = parts.len();
        parts.into_iter().take(len - 1).collect()
    }

    fn find_longest_prefix(&self) -> String {
        let sections = self.report.sections.clone();
        let file_paths: Vec<Vec<&str>> = sections
            .keys()
            .map(|key| Self::drop_file_name(key.source_file.to_str().unwrap()))
            .collect();

        if file_paths.is_empty() {
            return String::new();
        }

        let mut prefix = file_paths[0].clone();
        for path in file_paths.iter().skip(1) {
            let longest_prefix = prefix
                .iter()
                .zip(path.iter())
                .take_while(|(a, b)| a == b)
                .map(|(a, _)| *a)
                .collect::<Vec<&str>>();
            if longest_prefix.is_empty() {
                return String::new();
            }
            prefix = longest_prefix;
        }

        return prefix.join("/");
    }

    pub fn with_longest_prefix(&self) -> AggregatorInput {
        let prefix = self.find_longest_prefix();
        self.with_prefix(&prefix)
    }
}

#[cfg(test)]
mod test {
    use crate::{aggregation::input::AggregatorInput, test_utils::builders::InsertSection};

    #[test]
    fn test_new_list_sections() {
        let report = lcov::report::Report::new();
        let input = AggregatorInput::new(report.clone());
        assert_eq!(input.list_sections(), report.sections);
    }

    #[test]
    fn test_with_prefix_list_sections() {
        let report = lcov::report::Report::new().insert_empty_section("my/very/long/path/file.cpp");
        let input = AggregatorInput::new(report.clone());
        let new_input = input.with_prefix("my/very/long/path/");
        let new_sections = new_input.list_sections();
        assert_eq!(new_sections.len(), 1);
        assert_eq!(
            new_sections.keys().next().unwrap().source_file,
            std::path::PathBuf::from("file.cpp")
        );
    }

    #[test]
    fn test_prefix_not_found_list_sectiosn() {
        let report = lcov::report::Report::new().insert_empty_section("my/very/long/path/file.cpp");
        let input = AggregatorInput::new(report.clone());
        let new_input = input.with_prefix("my/very/long/path2/");
        let new_sections = new_input.list_sections();
        assert_eq!(new_sections.len(), 1);
        assert_eq!(
            new_sections.keys().next().unwrap().source_file,
            std::path::PathBuf::from("my/very/long/path/file.cpp")
        );
    }

    #[test]
    fn test_longest_prefix_of_empty_is_empty() {
        let report = lcov::report::Report::new();
        let input = AggregatorInput::new(report.clone());
        assert_eq!(input.find_longest_prefix(), "");
    }

    #[test]
    fn test_longest_prefix_of_single_file_is_full_path() {
        let report = lcov::report::Report::new().insert_empty_section("my/very/long/path/file.cpp");
        let input = AggregatorInput::new(report.clone());
        assert_eq!(input.find_longest_prefix(), "my/very/long/path");
    }

    #[test]
    fn test_longest_prefix_of_multiple_files() {
        let report = lcov::report::Report::new()
            .insert_empty_section("my/very/long/path/file.cpp")
            .insert_empty_section("my/very/long/path/file2.cpp");
        let input = AggregatorInput::new(report.clone());
        assert_eq!(input.find_longest_prefix(), "my/very/long/path");
    }

    #[test]
    fn test_longest_prefix_of_multiple_files_at_different_levels() {
        let report = lcov::report::Report::new()
            .insert_empty_section("my/very/long/path/file.cpp")
            .insert_empty_section("my/very/long/path2/file2.cpp");
        let input = AggregatorInput::new(report.clone());
        assert_eq!(input.find_longest_prefix(), "my/very/long");
    }
}
