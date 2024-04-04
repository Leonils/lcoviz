use std::path::PathBuf;

#[derive(Debug, PartialEq, Default)]
struct ReportTree {
    modules: Vec<TestedModule>,
    source_files: Vec<TestedFile>,
}

impl ReportTree {
    fn split_path(path: PathBuf) -> Vec<String> {
        path.iter()
            .map(|p| p.to_str().unwrap().to_string())
            .collect()
    }

    pub fn from_original_report(report: lcov::report::Report) -> Self {
        let mut tree = ReportTree::default();
        for (section_key, section_value) in report.sections {
            let section_path = Self::split_path(section_key.source_file);

            let file_name = section_path[section_path.len() - 1].clone();
            let file = TestedFile { file_name };

            if section_path.is_empty() {
                println!("Empty path");
                continue;
            }

            if section_path.len() == 1 {
                tree.source_files.push(file);
                continue;
            }

            let module_name = section_path[0].clone();
            let module_path_queue = section_path[1..section_path.len() - 1].to_vec();
            if let Some(existing_module) = tree.modules.iter_mut().find(|m| m.name == module_name) {
                existing_module.add_file(module_path_queue, file);
            } else {
                let mut module = TestedModule::new(module_name);
                module.add_file(module_path_queue, file);
                tree.modules.push(module);
            }
        }

        tree
    }
}

#[derive(Debug, PartialEq, Default)]
struct TestedFile {
    file_name: String,
}

#[derive(Debug, PartialEq, Default)]
struct TestedModule {
    name: String,
    source_files: Vec<TestedFile>,
    modules: Vec<TestedModule>,
}

impl TestedModule {
    fn new(name: String) -> Self {
        TestedModule {
            name,
            source_files: vec![],
            modules: vec![],
        }
    }

    fn add_file(&mut self, path: Vec<String>, file: TestedFile) {
        if path.is_empty() {
            self.source_files.push(file);
            return;
        }

        let module_name = path[0].clone();
        if let Some(existing_module) = self.modules.iter_mut().find(|m| m.name == module_name) {
            existing_module.add_file(path[1..].to_vec(), file);
            return;
        }

        let module = TestedModule::new(module_name);
        self.modules.push(module);
        self.modules
            .last_mut()
            .unwrap()
            .add_file(path[1..].to_vec(), file);
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::{ReportTree, TestedFile, TestedModule};
    use lcov::report::section::{Key as SectionKey, Value as SectionValue};

    trait FromStr {
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

    trait FromSourceFile {
        fn from_source_files(name: &str, source_files: Vec<TestedFile>) -> Self;
    }
    trait FromModules {
        fn from_modules(name: &str, modules: Vec<TestedModule>) -> Self;
    }

    impl TestedFile {
        fn new(file_name: &str) -> Self {
            TestedFile {
                file_name: String::from(file_name),
            }
        }
    }
    impl FromSourceFile for TestedModule {
        fn from_source_files(name: &str, source_files: Vec<TestedFile>) -> Self {
            TestedModule {
                name: String::from(name),
                source_files,
                modules: vec![],
            }
        }
    }
    impl FromModules for TestedModule {
        fn from_modules(name: &str, modules: Vec<TestedModule>) -> Self {
            TestedModule {
                name: String::from(name),
                source_files: vec![],
                modules,
            }
        }
    }
    impl FromSourceFile for ReportTree {
        fn from_source_files(_name: &str, source_files: Vec<TestedFile>) -> Self {
            ReportTree {
                source_files,
                ..Default::default()
            }
        }
    }
    impl FromModules for ReportTree {
        fn from_modules(_name: &str, modules: Vec<TestedModule>) -> Self {
            ReportTree {
                modules,
                ..Default::default()
            }
        }
    }

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
        original_report
            .sections
            .insert(SectionKey::from_str("main.cpp"), SectionValue::default());

        let report_tree = ReportTree::from_original_report(original_report);

        let tested_file = TestedFile::new("main.cpp");
        let expected_tree = ReportTree::from_source_files("report", vec![tested_file]);

        assert_eq!(expected_tree, report_tree);
    }

    #[test]
    fn when_building_tree_with_a_report_with_one_file_nested_it_should_get_a_report_with_module_node(
    ) {
        let mut original_report = lcov::report::Report::new();
        original_report.sections.insert(
            SectionKey::from_str("package/main.cpp"),
            SectionValue::default(),
        );

        let report_tree = ReportTree::from_original_report(original_report);

        let tested_file = TestedFile::new("main.cpp");
        let tested_module = TestedModule::from_source_files("package", vec![tested_file]);
        let expected_tree = ReportTree::from_modules("report", vec![tested_module]);

        assert_eq!(expected_tree, report_tree);
    }

    #[test]
    fn when_building_tree_with_a_report_with_one_file_deeply_nested_it_should_get_a_report_with_module_node(
    ) {
        let mut original_report = lcov::report::Report::new();
        original_report.sections.insert(
            SectionKey::from_str("package/sub-package/main.cpp"),
            SectionValue::default(),
        );

        let report_tree = ReportTree::from_original_report(original_report);

        let tested_file = TestedFile::new("main.cpp");
        let tested_module_2 = TestedModule::from_source_files("sub-package", vec![tested_file]);
        let tested_module_1 = TestedModule::from_modules("package", vec![tested_module_2]);
        let expected_tree = ReportTree::from_modules("report", vec![tested_module_1]);

        assert_eq!(expected_tree, report_tree);
    }

    #[test]
    fn when_building_tree_with_report_2_files_at_root_shall_have_2_files_in_report_tree() {
        let mut original_report = lcov::report::Report::new();
        original_report
            .sections
            .insert(SectionKey::from_str("main.cpp"), SectionValue::default());
        original_report
            .sections
            .insert(SectionKey::from_str("module.cpp"), SectionValue::default());

        let report_tree = ReportTree::from_original_report(original_report);

        let tested_file_main = TestedFile::new("main.cpp");
        let tested_file_module = TestedFile::new("module.cpp");
        let expected_tree =
            ReportTree::from_source_files("report", vec![tested_file_main, tested_file_module]);

        assert_eq!(expected_tree, report_tree);
    }

    #[test]
    fn when_building_tree_with_report_2_files_deeply_nested_shall_have_2_files_in_report_tree() {
        let mut original_report = lcov::report::Report::new();
        original_report.sections.insert(
            SectionKey::from_str("my/package/main.cpp"),
            SectionValue::default(),
        );
        original_report.sections.insert(
            SectionKey::from_str("my/package/module.cpp"),
            SectionValue::default(),
        );

        let report_tree = ReportTree::from_original_report(original_report);

        let tested_file_main = TestedFile::new("main.cpp");
        let tested_file_module = TestedFile::new("module.cpp");
        let tested_module_package =
            TestedModule::from_source_files("package", vec![tested_file_main, tested_file_module]);
        let tested_module_my = TestedModule::from_modules("my", vec![tested_module_package]);
        let expected_tree = ReportTree::from_modules("report", vec![tested_module_my]);

        assert_eq!(expected_tree, report_tree);
    }

    #[test]
    fn when_building_tree_with_2_files_in_different_packages_both_packages_shall_exist_in_tree() {
        let mut original_report = lcov::report::Report::new();
        original_report.sections.insert(
            SectionKey::from_str("my/package/main.cpp"),
            SectionValue::default(),
        );
        original_report.sections.insert(
            SectionKey::from_str("yours/module.cpp"),
            SectionValue::default(),
        );

        let report_tree = ReportTree::from_original_report(original_report);

        let tested_file_main = TestedFile::new("main.cpp");
        let tested_file_module = TestedFile::new("module.cpp");
        let tested_module_package =
            TestedModule::from_source_files("package", vec![tested_file_main]);
        let tested_module_my = TestedModule::from_modules("my", vec![tested_module_package]);
        let tested_module_yours =
            TestedModule::from_source_files("yours", vec![tested_file_module]);
        let expected_tree =
            ReportTree::from_modules("report", vec![tested_module_my, tested_module_yours]);

        assert_eq!(expected_tree, report_tree);
    }
}
