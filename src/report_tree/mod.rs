use std::path::PathBuf;

use self::{tested_file::TestedFile, tested_module::TestedModule};

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
            let file = TestedFile::new(&file_name, &section_path.join("/"));

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
            if let Some(existing_module) = tree
                .modules
                .iter_mut()
                .find(|m| m.get_name() == module_name)
            {
                existing_module.add_file(module_path_queue, file);
            } else {
                let mut module = TestedModule::new(module_name.clone(), module_name);
                module.add_file(module_path_queue, file);
                tree.modules.push(module);
            }
        }

        tree
    }
}

mod tested_file {
    use super::with_path::WithPath;

    #[derive(Debug, PartialEq, Default)]
    pub struct TestedFile {
        file_name: String,
        path: String,
    }

    impl TestedFile {
        pub fn new(path: &str, file_name: &str) -> Self {
            TestedFile {
                file_name: String::from(file_name),
                path: String::from(path),
            }
        }
    }

    impl WithPath for TestedFile {
        fn get_path_string(&self) -> String {
            self.path.clone()
        }
    }
}

mod tested_module {
    use super::{tested_file::TestedFile, with_path::WithPath};

    #[derive(Debug, PartialEq, Default)]
    pub struct TestedModule {
        name: String,
        path: String,
        source_files: Vec<TestedFile>,
        modules: Vec<TestedModule>,
    }

    impl TestedModule {
        pub fn new(path: String, name: String) -> Self {
            TestedModule {
                name,
                path,
                source_files: vec![],
                modules: vec![],
            }
        }

        pub fn get_name(&self) -> String {
            self.name.clone()
        }

        pub fn add_file(&mut self, path: Vec<String>, file: TestedFile) {
            if path.is_empty() {
                self.source_files.push(file);
                return;
            }

            let module_name = path[0].clone();
            if let Some(existing_module) = self.modules.iter_mut().find(|m| m.name == module_name) {
                existing_module.add_file(path[1..].to_vec(), file);
                return;
            }

            let module = TestedModule::new(format!("{}/{}", self.path, module_name), module_name);
            self.modules.push(module);
            self.modules
                .last_mut()
                .unwrap()
                .add_file(path[1..].to_vec(), file);
        }
    }

    impl WithPath for TestedModule {
        fn get_path_string(&self) -> String {
            self.path.clone()
        }
    }

    #[cfg(test)]
    impl TestedModule {
        pub fn from_source_files(path: &str, name: &str, source_files: Vec<TestedFile>) -> Self {
            TestedModule {
                name: String::from(name),
                path: String::from(path),
                source_files,
                modules: vec![],
            }
        }

        pub fn from_modules(path: &str, name: &str, modules: Vec<TestedModule>) -> Self {
            TestedModule {
                name: String::from(name),
                path: String::from(path),
                source_files: vec![],
                modules,
            }
        }

        pub fn get_module_at(&self, i: usize) -> &TestedModule {
            self.modules.get(i).unwrap()
        }

        pub fn get_source_file_at(&self, i: usize) -> &TestedFile {
            self.source_files.get(i).unwrap()
        }
    }
}

mod with_path {
    pub trait WithPath {
        fn get_path_string(&self) -> String;
        fn get_path(&self) -> Vec<String> {
            self.get_path_string()
                .split('/')
                .map(|s| s.to_string())
                .collect()
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::report_tree::with_path::WithPath;

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
        fn from_source_files(path: &str, name: &str, source_files: Vec<TestedFile>) -> Self;
    }
    trait FromModules {
        fn from_modules(path: &str, name: &str, modules: Vec<TestedModule>) -> Self;
    }

    impl ReportTree {
        fn from_source_files(source_files: Vec<TestedFile>) -> Self {
            ReportTree {
                source_files,
                ..Default::default()
            }
        }
    }
    impl ReportTree {
        fn from_modules(modules: Vec<TestedModule>) -> Self {
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

        let tested_file = TestedFile::new("main.cpp", "main.cpp");
        let expected_tree = ReportTree::from_source_files(vec![tested_file]);

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

        let tested_file = TestedFile::new("package/main.cpp", "main.cpp");
        let tested_module =
            TestedModule::from_source_files("package", "package", vec![tested_file]);
        let expected_tree = ReportTree::from_modules(vec![tested_module]);

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

        let tested_file = TestedFile::new("package/sub-package/main.cpp", "main.cpp");
        let tested_module_2 = TestedModule::from_source_files(
            "package/sub-package",
            "sub-package",
            vec![tested_file],
        );
        let tested_module_1 =
            TestedModule::from_modules("package", "package", vec![tested_module_2]);
        let expected_tree = ReportTree::from_modules(vec![tested_module_1]);

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

        let tested_file_main = TestedFile::new("main.cpp", "main.cpp");
        let tested_file_module = TestedFile::new("module.cpp", "module.cpp");
        let expected_tree =
            ReportTree::from_source_files(vec![tested_file_main, tested_file_module]);

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

        let tested_file_main = TestedFile::new("my/package/main.cpp", "main.cpp");
        let tested_file_module = TestedFile::new("my/package/module.cpp", "module.cpp");
        let tested_module_package = TestedModule::from_source_files(
            "my/package",
            "package",
            vec![tested_file_main, tested_file_module],
        );
        let tested_module_my = TestedModule::from_modules("my", "my", vec![tested_module_package]);
        let expected_tree = ReportTree::from_modules(vec![tested_module_my]);

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

        let tested_file_main = TestedFile::new("my/package/main.cpp", "main.cpp");
        let tested_file_module = TestedFile::new("yours/module.cpp", "module.cpp");
        let tested_module_package =
            TestedModule::from_source_files("my/package", "package", vec![tested_file_main]);
        let tested_module_my = TestedModule::from_modules("my", "my", vec![tested_module_package]);
        let tested_module_yours =
            TestedModule::from_source_files("yours", "yours", vec![tested_file_module]);
        let expected_tree = ReportTree::from_modules(vec![tested_module_my, tested_module_yours]);

        assert_eq!(expected_tree, report_tree);
    }

    #[test]
    fn when_building_tree_with_1_file_deeply_nested_i_can_access_path_of_modules_along_path_to_file(
    ) {
        let mut original_report = lcov::report::Report::new();
        original_report.sections.insert(
            SectionKey::from_str("package/sub-package/main.cpp"),
            SectionValue::default(),
        );

        let report_tree = ReportTree::from_original_report(original_report);

        let package = report_tree.modules.get(0).unwrap();
        let sub_package = package.get_module_at(0);
        let file = sub_package.get_source_file_at(0);

        assert_eq!(file.get_path(), vec!["package", "sub-package", "main.cpp"]);
        assert_eq!(sub_package.get_path(), vec!["package", "sub-package"]);
        assert_eq!(package.get_path(), vec!["package"]);
    }
}
