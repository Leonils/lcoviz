use lcov::{
    report::section::{Key as SectionKey, Value as SectionValue},
    Reader, Report as LcovReport,
};
use std::collections::{BTreeMap, HashMap};

use crate::core::FileSystem;

use super::config::Input;

pub struct AggregatorInput {
    report: lcov::report::Report,
    prefix: String,
    name: Option<String>,
    key: String,
}

impl AggregatorInput {
    pub fn new(report: LcovReport) -> Self {
        Self {
            report,
            prefix: String::new(),
            key: String::new(),
            name: None,
        }
    }

    pub fn from_config_input(input: Input, fs: &impl FileSystem) -> Self {
        let report_content = fs.read_to_string(&input.path).unwrap();
        let reader = Reader::new(report_content.as_bytes());
        let report = LcovReport::from_reader(reader).unwrap();
        let aggregator_input = Self::new(report);
        let aggregator_input = match input.prefix {
            Some(prefix) => aggregator_input.with_prefix(prefix.to_str().unwrap()),
            None => aggregator_input.with_longest_prefix(),
        };

        match input.name {
            Some(name) => aggregator_input.with_name(&name),
            None => aggregator_input,
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
            name: self.name,
        }
    }

    pub fn with_key(self, key: &str) -> AggregatorInput {
        AggregatorInput {
            report: self.report,
            prefix: self.prefix,
            key: key.to_string(),
            name: self.name,
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
        AggregatorInput { prefix, ..self }
    }

    pub fn with_name(self, name: &str) -> AggregatorInput {
        AggregatorInput {
            name: Some(name.to_string()),
            ..self
        }
    }

    pub fn get_prefix(&self) -> &str {
        &self.prefix
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_name(&self) -> &str {
        self.name
            .as_deref()
            .unwrap_or_else(|| self.last_part_of_prefix())
    }

    pub fn last_part_of_prefix(&self) -> &str {
        self.prefix.split('/').last().unwrap_or("")
    }

    pub fn build_from_inputs(inputs: Vec<Input>, fs: &impl FileSystem) -> Vec<AggregatorInput> {
        let mut report_names = HashMap::<String, u32>::new();
        let mut report_inputs = Vec::<AggregatorInput>::new();

        for config_input in inputs.into_iter() {
            let aggregator_input = AggregatorInput::from_config_input(config_input, fs);
            let wanted_key = aggregator_input.last_part_of_prefix().to_string();
            report_names
                .entry(wanted_key)
                .and_modify(|e| *e += 1)
                .or_insert(1);
            report_inputs.push(aggregator_input);
        }

        let mut inputs_with_key = Vec::<AggregatorInput>::new();
        let mut dedup_counters = HashMap::<String, u32>::new();
        for input in report_inputs {
            let key = input.last_part_of_prefix().to_string();
            let count = report_names.get(&key).unwrap().to_owned();
            let key = if count > 1 {
                let c = dedup_counters
                    .entry(key.clone())
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
                format!("{}_{}", key, c)
            } else {
                key
            };
            inputs_with_key.push(input.with_key(&key));
        }

        inputs_with_key
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{core::MockFileSystem, test_utils::builders::InsertSection};
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

    #[test]
    fn when_creating_aggregator_inputs_from_one_config_input_then_key_is_last_part_of_prefix() {
        let mut fs = MockFileSystem::new();
        fs.expect_read_to_string().returning(|_| Ok("".to_string()));
        let input = vec![Input::from_name_prefix_and_path(
            "Lib".into(),
            "project/package_1/src".into(),
            "./report.info".into(),
        )];
        let aggregator_input = AggregatorInput::build_from_inputs(input, &fs);

        assert_eq!(aggregator_input.len(), 1);
        assert_eq!(aggregator_input[0].get_key(), "src");
    }

    #[test]
    fn when_creating_aggregator_inputs_from_two_config_input_with_same_key_then_key_deduplicated() {
        let mut fs = MockFileSystem::new();
        fs.expect_read_to_string().returning(|_| Ok("".to_string()));
        let input = vec![
            Input::from_name_prefix_and_path(
                "Lib".into(),
                "project/package_1/src".into(),
                "./report.info".into(),
            ),
            Input::from_name_prefix_and_path(
                "Lib".into(),
                "project/package_2/src".into(),
                "./report.info".into(),
            ),
        ];
        let aggregator_input = AggregatorInput::build_from_inputs(input, &fs);

        assert_eq!(aggregator_input.len(), 2);
        assert_eq!(aggregator_input[0].get_key(), "src_1");
        assert_eq!(aggregator_input[1].get_key(), "src_2");
    }
}
