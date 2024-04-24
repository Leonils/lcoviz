use lcov::report::section::{Key as SectionKey, Value as SectionValue};
use std::collections::BTreeMap;

pub struct AggregatorInput {
    report: lcov::report::Report,
    prefix: String,
    key: String,
}

impl AggregatorInput {
    pub fn new(report: lcov::report::Report) -> Self {
        Self {
            report,
            prefix: String::new(),
            key: String::new(),
        }
    }

    pub fn list_sections(&self) -> BTreeMap<SectionKey, SectionValue> {
        self.report.sections.clone()
    }

    pub fn with_prefix(self, prefix: &str) -> AggregatorInput {
        let prefix_parts = prefix
            .split('/')
            .filter(|part| !part.is_empty())
            .collect::<Vec<&str>>();
        for (key, _) in self.report.sections.iter() {
            let source_file_parts = key
                .source_file
                .to_str()
                .unwrap()
                .split('/')
                .collect::<Vec<&str>>();

            if !source_file_parts.starts_with(&prefix_parts) {
                panic!(
                    "Some tested files do not start with the prefix '{}'. For example, {}",
                    prefix,
                    key.source_file.to_str().unwrap()
                );
            }
        }

        AggregatorInput {
            report: self.report,
            prefix: prefix.to_string(),
            key: self.key,
        }
    }

    pub fn with_key(self, key: &str) -> AggregatorInput {
        AggregatorInput {
            report: self.report,
            prefix: self.prefix,
            key: key.to_string(),
        }
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

    pub fn with_longest_prefix(self) -> AggregatorInput {
        let prefix = self.find_longest_prefix();
        AggregatorInput {
            report: self.report,
            prefix,
            key: self.key,
        }
    }

    pub fn get_prefix(&self) -> &str {
        &self.prefix
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn last_part_of_prefix(&self) -> &str {
        self.prefix.split('/').last().unwrap_or("")
    }
}

#[cfg(test)]
mod test {
    use crate::{aggregation::input::AggregatorInput, test_utils::builders::InsertSection};
    use lcov::Report;

    #[test]
    fn test_new_list_sections() {
        let report = Report::new();
        let input = AggregatorInput::new(report.clone());
        assert_eq!(input.list_sections(), report.sections);
    }

    #[test]
    fn test_with_prefix_list_sections() {
        let report = Report::new().insert_empty_section("my/very/long/path/file.cpp");
        let input = AggregatorInput::new(report.clone());
        let new_input = input.with_prefix("my/very/long/path/");
        assert_eq!(new_input.prefix, "my/very/long/path/");
    }

    #[test]
    #[should_panic(
        expected = "Some tested files do not start with the prefix 'my/very/long/path/'. For example, another/prefix/file.cpp"
    )]
    fn test_with_prefix_list_sections_with_invalid_prefix() {
        let report = Report::new().insert_empty_section("another/prefix/file.cpp");
        let input = AggregatorInput::new(report.clone());
        input.with_prefix("my/very/long/path/");
    }

    #[test]
    fn test_longest_prefix_of_empty_is_empty() {
        let report = Report::new();
        let input = AggregatorInput::new(report.clone());
        assert_eq!(input.find_longest_prefix(), "");
    }

    #[test]
    fn test_longest_prefix_of_single_file_is_full_path() {
        let report = Report::new().insert_empty_section("my/very/long/path/file.cpp");
        let input = AggregatorInput::new(report.clone());
        assert_eq!(input.find_longest_prefix(), "my/very/long/path");
    }

    #[test]
    fn test_longest_prefix_of_multiple_files() {
        let report = Report::new()
            .insert_empty_section("my/very/long/path/file.cpp")
            .insert_empty_section("my/very/long/path/file2.cpp");
        let input = AggregatorInput::new(report.clone());
        assert_eq!(input.find_longest_prefix(), "my/very/long/path");
    }

    #[test]
    fn test_longest_prefix_of_multiple_files_at_different_levels() {
        let report = Report::new()
            .insert_empty_section("my/very/long/path/file.cpp")
            .insert_empty_section("my/very/long/path2/file2.cpp");
        let input = AggregatorInput::new(report.clone());
        assert_eq!(input.find_longest_prefix(), "my/very/long");
    }

    #[test]
    fn test_with_longest_prefix() {
        let report = Report::new()
            .insert_empty_section("my/very/long/path/file.cpp")
            .insert_empty_section("my/very/long/path/file2.cpp");
        let input = AggregatorInput::new(report.clone());
        let new_input = input.with_longest_prefix();
        assert_eq!(new_input.prefix, "my/very/long/path");
    }

    #[test]
    fn last_part_of_empty_prefix_is_empty() {
        let report = Report::new();
        let input = AggregatorInput::new(report.clone());
        assert_eq!(input.last_part_of_prefix(), "");
    }

    #[test]
    fn last_part_of_prefix_with_single_part_is_single_part() {
        let report = Report::new();
        let input = AggregatorInput::new(report.clone()).with_prefix("foo");
        assert_eq!(input.last_part_of_prefix(), "foo");
    }

    #[test]
    fn last_part_of_absolute_prefix_with_single_part_is_single_part() {
        let report = Report::new();
        let input = AggregatorInput::new(report.clone()).with_prefix("/foo");
        assert_eq!(input.last_part_of_prefix(), "foo");
    }

    #[test]
    fn last_part_of_absolute_prefix_with_multiple_part_is_last_part() {
        let report = Report::new();
        let input = AggregatorInput::new(report.clone()).with_prefix("/foo/bar");
        assert_eq!(input.last_part_of_prefix(), "bar");
    }
}
