#[derive(Debug, PartialEq, Default)]
struct ReportTree {
    source_files: Vec<TestedFile>,
}

impl ReportTree {
    pub fn from_original_report(report: lcov::report::Report) -> Self {
        let mut tree = ReportTree::default();
        for (section_key, section_value) in report.sections {
            tree.source_files.push(TestedFile {
                file_name: section_key
                    .source_file
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            })
        }

        tree
    }
}

#[derive(Debug, PartialEq, Default)]
struct TestedFile {
    file_name: String,
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::{ReportTree, TestedFile};

    #[test]
    fn when_building_tree_with_an_empty_report_it_should_get_an_empty_report() {
        let original_report = lcov::report::Report::new();
        let report_tree = ReportTree::from_original_report(original_report);
        assert_eq!(ReportTree::default(), report_tree);
    }

    #[test]
    fn when_building_tree_with_a_report_with_one_file_it_should_get_a_report_with_tested_file_child(
    ) {
        let mut original_report = lcov::report::Report::new();
        original_report.sections.insert(
            lcov::report::section::Key {
                source_file: PathBuf::from("main.cpp"),
                test_name: String::from(""),
            },
            lcov::report::section::Value::default(),
        );

        let report_tree = ReportTree::from_original_report(original_report);
        let mut expected_tree = ReportTree::default();
        expected_tree.source_files.push(TestedFile {
            file_name: String::from("main.cpp"),
        });
        assert_eq!(expected_tree, report_tree);
    }
}
