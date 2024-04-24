use std::path::PathBuf;

use lcov::report::section::{Key as SectionKey, Value as SectionValue};

use crate::core::{AggregatedCoverage, TestedContainer, TestedFile, WithPath};

use super::{input::AggregatorInput, tested_file::TestedCodeFile, tested_module::TestedModule};

#[derive(Debug, PartialEq, Default)]
pub struct TestedRoot {
    key: String,
    name: String,
    modules: Vec<TestedModule>,
    source_files: Vec<TestedCodeFile>,
    aggregated: AggregatedCoverage,
    prefix: PathBuf,
}

impl TestedRoot {
    pub fn new(args: AggregatorInput) -> Self {
        let prefix_path = PathBuf::from(args.get_prefix());
        let name = prefix_path
            .components()
            .last()
            .map(|c| c.as_os_str().to_str())
            .flatten()
            .unwrap_or("Test report");

        let mut tree = TestedRoot {
            aggregated: AggregatedCoverage::default(),
            prefix: prefix_path.to_owned(),
            name: name.to_string(),
            key: args.get_key().to_string(),
            ..Default::default()
        };

        for (section_key, section_value) in args.list_sections() {
            tree.add_file(section_key, section_value, args.get_prefix());
        }

        tree
    }

    fn find_module_by_name(&mut self, module_name: &str) -> Option<&mut TestedModule> {
        self.modules
            .iter_mut()
            .find(|m| m.get_name() == module_name)
    }

    fn insert_new_module(&mut self, module_path: &str, module_name: &str) -> &mut TestedModule {
        let module = TestedModule::new(module_path.to_string(), module_name.to_string());
        self.modules.push(module);
        self.modules.last_mut().unwrap()
    }

    fn add_file(&mut self, section_key: SectionKey, section_value: SectionValue, prefix: &str) {
        let file = TestedCodeFile::from_section(section_key, section_value, prefix, &self.key);
        let path_relative_to_root = file
            .get_path_relative_to(&self.get_path())
            .components()
            .filter(|c| c.as_os_str() != "/")
            .map(|c| c.as_os_str().to_str().unwrap().to_string())
            .collect::<Vec<String>>();

        if path_relative_to_root.is_empty() {
            println!("Empty path");
            return;
        }

        self.aggregated.add(&file.get_aggregated_coverage());
        if path_relative_to_root.len() == 1 {
            self.source_files.push(file);
            return;
        }

        let module_name = path_relative_to_root[0].clone();
        let module_path_queue = path_relative_to_root[1..path_relative_to_root.len() - 1].to_vec();

        println!(
            "module_path_queue: {:?}, module_name: {:?}, path_relative: {:?}",
            module_path_queue, module_name, path_relative_to_root
        );

        let target_module = match self.find_module_by_name(&module_name) {
            Some(existing_module) => existing_module,
            None => self.insert_new_module(
                PathBuf::new()
                    .join(&self.key)
                    .join(&module_name)
                    .to_str()
                    .unwrap(),
                module_name.as_str(),
            ),
        };

        target_module.add_file(module_path_queue, file);
    }

    pub fn enumerate_code_files(&self) -> impl Iterator<Item = &TestedCodeFile> {
        self.modules
            .iter()
            .flat_map(|m| m.enumerate_code_files())
            .chain(self.source_files.iter())
    }

    pub fn get_prefix(&self) -> &PathBuf {
        &self.prefix
    }
}

impl TestedContainer for TestedRoot {
    fn get_aggregated_coverage(&self) -> &AggregatedCoverage {
        &self.aggregated
    }

    fn get_container_children(&self) -> impl Iterator<Item = &impl TestedContainer> {
        self.modules.iter()
    }

    fn get_code_file_children(&self) -> impl Iterator<Item = &impl TestedFile> {
        self.source_files.iter()
    }
}

impl WithPath for TestedRoot {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn is_dir(&self) -> bool {
        true
    }

    fn get_path_string(&self) -> String {
        self.key.to_string()
    }
}

#[cfg(test)]
impl TestedRoot {
    pub fn from_source_files(source_files: Vec<TestedCodeFile>) -> Self {
        TestedRoot {
            aggregated: AggregatedCoverage::default(),
            modules: vec![],
            source_files,
            prefix: PathBuf::from(""),
            name: "Test report".to_string(),
            key: "".to_string(),
        }
    }

    pub fn from_modules(modules: Vec<TestedModule>) -> Self {
        TestedRoot {
            aggregated: AggregatedCoverage::default(),
            modules,
            source_files: vec![],
            prefix: PathBuf::from(""),
            name: "Test report".to_string(),
            key: "".to_string(),
        }
    }

    pub fn from_source_files_and_modules(
        source_files: Vec<TestedCodeFile>,
        modules: Vec<TestedModule>,
    ) -> Self {
        TestedRoot {
            aggregated: AggregatedCoverage::default(),
            modules,
            source_files,
            prefix: PathBuf::from(""),
            name: "Test report".to_string(),
            key: "".to_string(),
        }
    }

    pub fn from_original_report(report: lcov::report::Report) -> Self {
        TestedRoot::new(AggregatorInput::new(report))
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::{
        aggregation::aggregated::assert_aggregated_counters_eq,
        core::{TestedContainer, TestedFile, WithPath},
        test_utils::builders::{
            generate_2_lines_1_covered_section, generate_3_lines_2_covered_section, InsertSection,
        },
    };

    use super::{
        super::{tested_file::TestedCodeFile, tested_module::TestedModule},
        TestedRoot,
    };
    use lcov::report::Report as LcovReport;

    #[test]
    fn when_building_tree_with_an_empty_report_it_should_get_an_empty_report() {
        let original_report = LcovReport::new();
        let report_tree = TestedRoot::from_original_report(original_report);
        assert_eq!(
            TestedRoot {
                name: "Test report".to_string(),
                ..TestedRoot::default()
            },
            report_tree
        );
    }

    #[test]
    fn when_building_tree_with_a_report_with_one_file_it_should_get_a_report_with_tested_file_child(
    ) {
        let original_report = LcovReport::new().insert_empty_section("main.cpp");

        let report_tree = TestedRoot::from_original_report(original_report);

        let tested_file = TestedCodeFile::new("main.cpp", "main.cpp");
        let expected_tree = TestedRoot::from_source_files(vec![tested_file]);

        assert_eq!(expected_tree, report_tree);
    }

    #[test]
    fn when_building_tree_with_a_report_with_one_file_nested_it_should_get_a_report_with_module_node(
    ) {
        let original_report = LcovReport::new().insert_empty_section("package/main.cpp");
        let report_tree = TestedRoot::from_original_report(original_report);

        let tested_file = TestedCodeFile::new("package/main.cpp", "main.cpp");
        let tested_module =
            TestedModule::from_source_files("package", "package", vec![tested_file]);
        let expected_tree = TestedRoot::from_modules(vec![tested_module]);

        assert_eq!(expected_tree, report_tree);
    }

    #[test]
    fn when_building_tree_with_a_report_with_path_starting_with_slash_should_start_with_first_module(
    ) {
        let original_report = LcovReport::new().insert_empty_section("/package/main.cpp");
        let report_tree = TestedRoot::from_original_report(original_report);

        let tested_file = TestedCodeFile::new("/package/main.cpp", "main.cpp");
        let tested_module =
            TestedModule::from_source_files("package", "package", vec![tested_file]);
        let expected_tree = TestedRoot::from_modules(vec![tested_module]);

        assert_eq!(expected_tree, report_tree);
    }

    #[test]
    fn when_building_tree_with_a_report_with_one_file_deeply_nested_it_should_get_a_report_with_module_node(
    ) {
        let original_report =
            LcovReport::new().insert_empty_section("package/sub-package/main.cpp");

        let report_tree = TestedRoot::from_original_report(original_report);

        let tested_file = TestedCodeFile::new("package/sub-package/main.cpp", "main.cpp");
        let tested_module_2 = TestedModule::from_source_files(
            "package/sub-package",
            "sub-package",
            vec![tested_file],
        );
        let tested_module_1 =
            TestedModule::from_modules("package", "package", vec![tested_module_2]);
        let expected_tree = TestedRoot::from_modules(vec![tested_module_1]);

        assert_eq!(expected_tree, report_tree);
    }

    #[test]
    fn when_building_tree_with_report_2_files_at_root_shall_have_2_files_in_report_tree() {
        let original_report = LcovReport::new()
            .insert_empty_section("main.cpp")
            .insert_empty_section("module.cpp");

        let report_tree = TestedRoot::from_original_report(original_report);

        let tested_file_main = TestedCodeFile::new("main.cpp", "main.cpp");
        let tested_file_module = TestedCodeFile::new("module.cpp", "module.cpp");
        let expected_tree =
            TestedRoot::from_source_files(vec![tested_file_main, tested_file_module]);

        assert_eq!(expected_tree, report_tree);
    }

    #[test]
    fn when_building_tree_with_report_2_files_deeply_nested_shall_have_2_files_in_report_tree() {
        let original_report = LcovReport::new()
            .insert_empty_section("my/package/main.cpp")
            .insert_empty_section("my/package/module.cpp");

        let report_tree = TestedRoot::from_original_report(original_report);

        let tested_file_main = TestedCodeFile::new("my/package/main.cpp", "main.cpp");
        let tested_file_module = TestedCodeFile::new("my/package/module.cpp", "module.cpp");
        let tested_module_package = TestedModule::from_source_files(
            "my/package",
            "package",
            vec![tested_file_main, tested_file_module],
        );
        let tested_module_my = TestedModule::from_modules("my", "my", vec![tested_module_package]);
        let expected_tree = TestedRoot::from_modules(vec![tested_module_my]);

        assert_eq!(expected_tree, report_tree);
    }

    #[test]
    fn when_building_tree_with_2_files_in_different_packages_both_packages_shall_exist_in_tree() {
        let original_report = LcovReport::new()
            .insert_empty_section("my/package/main.cpp")
            .insert_empty_section("yours/module.cpp");

        let report_tree = TestedRoot::from_original_report(original_report);

        let tested_file_main = TestedCodeFile::new("my/package/main.cpp", "main.cpp");
        let tested_file_module = TestedCodeFile::new("yours/module.cpp", "module.cpp");
        let tested_module_package =
            TestedModule::from_source_files("my/package", "package", vec![tested_file_main]);
        let tested_module_my = TestedModule::from_modules("my", "my", vec![tested_module_package]);
        let tested_module_yours =
            TestedModule::from_source_files("yours", "yours", vec![tested_file_module]);
        let expected_tree = TestedRoot::from_modules(vec![tested_module_my, tested_module_yours]);

        assert_eq!(expected_tree, report_tree);
    }

    #[test]
    fn when_building_tree_with_1_file_deeply_nested_i_can_access_path_of_modules_along_path_to_file(
    ) {
        let original_report =
            LcovReport::new().insert_empty_section("package/sub-package/main.cpp");
        let report_tree = TestedRoot::from_original_report(original_report);

        let package = report_tree.modules.get(0).unwrap();
        let sub_package = package.get_module_at(0);
        let file = sub_package.get_source_file_at(0);

        // assert_eq!(file.get_path(), vec!["package", "sub-package", "main.cpp"]);
        // assert_eq!(sub_package.get_path(), vec!["package", "sub-package"]);
        // assert_eq!(package.get_path(), vec!["package"]);

        assert_eq!(
            file.get_path(),
            PathBuf::from("package/sub-package/main.cpp")
        );

        assert_eq!(sub_package.get_path(), PathBuf::from("package/sub-package"));
        assert_eq!(package.get_path(), PathBuf::from("package"));
        assert_eq!(report_tree.get_path(), PathBuf::from(""));
    }

    #[test]
    fn when_building_tree_with_an_empty_report_it_should_get_0_aggregates() {
        let original_report = LcovReport::new();
        let report_tree = TestedRoot::from_original_report(original_report);

        assert_aggregated_counters_eq(&report_tree.aggregated.lines, 0, 0);
        assert_aggregated_counters_eq(&report_tree.aggregated.functions, 0, 0);
        assert_aggregated_counters_eq(&report_tree.aggregated.branches, 0, 0);
    }

    #[test]
    fn when_building_tree_with_a_top_level_file_with_coverage_data_it_should_get_the_same_aggregate(
    ) {
        let original_report =
            LcovReport::new().insert_section("main.cpp", generate_3_lines_2_covered_section());

        let report_tree = TestedRoot::from_original_report(original_report);
        let tested_file = report_tree.source_files.get(0).unwrap();

        assert_aggregated_counters_eq(&tested_file.get_aggregated_coverage().lines, 3, 2);
        assert_aggregated_counters_eq(&report_tree.aggregated.lines, 3, 2);
    }

    #[test]
    fn when_building_tree_with_a_nested_file_with_coverage_data_it_should_get_the_aggregate_all_along(
    ) {
        let original_report = LcovReport::new()
            .insert_section(
                "module1/sub-module/main.cpp",
                generate_3_lines_2_covered_section(),
            )
            .insert_section("module2/main.cpp", generate_2_lines_1_covered_section());

        // Aggregate
        let report_tree = TestedRoot::from_original_report(original_report);
        let module1 = report_tree.modules.get(0).unwrap();
        let sub_module1 = module1.get_module_at(0);
        let module2 = report_tree.modules.get(1).unwrap();

        assert_aggregated_counters_eq(&report_tree.aggregated.lines, 5, 3);
        assert_aggregated_counters_eq(&module1.get_aggregated_coverage().lines, 3, 2);
        assert_aggregated_counters_eq(&sub_module1.get_aggregated_coverage().lines, 3, 2);
        assert_aggregated_counters_eq(
            &sub_module1
                .get_source_file_at(0)
                .get_aggregated_coverage()
                .lines,
            3,
            2,
        );
        assert_aggregated_counters_eq(&module2.get_aggregated_coverage().lines, 2, 1);
        assert_aggregated_counters_eq(
            &module2
                .get_source_file_at(0)
                .get_aggregated_coverage()
                .lines,
            2,
            1,
        );
    }
}
