use lcov::report::section::{Key as SectionKey, Value as SectionValue};

use crate::core::AggregatedCoverage;

use super::{tested_file::TestedFile, tested_module::TestedModule, with_path::WithPath};

#[derive(Debug, PartialEq, Default)]
pub struct ReportTree {
    modules: Vec<TestedModule>,
    source_files: Vec<TestedFile>,
    aggregated: AggregatedCoverage,
}

impl ReportTree {
    pub fn from_original_report(report: lcov::report::Report) -> Self {
        let mut tree = ReportTree {
            aggregated: AggregatedCoverage::default(),
            ..Default::default()
        };

        for (section_key, section_value) in report.sections {
            tree.add_file(section_key, section_value)
        }

        tree
    }

    fn find_module_by_name(&mut self, module_name: &str) -> Option<&mut TestedModule> {
        self.modules
            .iter_mut()
            .find(|m| m.get_name() == module_name)
    }

    fn insert_new_module(&mut self, module_name: &str) -> &mut TestedModule {
        let module = TestedModule::new(module_name.to_string(), module_name.to_string());
        self.modules.push(module);
        self.modules.last_mut().unwrap()
    }

    fn add_file(&mut self, section_key: SectionKey, section_value: SectionValue) {
        let file = TestedFile::from_section(section_key, section_value);
        let section_path = file.get_path();

        if section_path.is_empty() {
            println!("Empty path");
            return;
        }

        self.aggregated.add(&file.aggregated);
        if section_path.len() == 1 {
            self.source_files.push(file);
            return;
        }

        let module_name = section_path[0].clone();
        let module_path_queue = section_path[1..section_path.len() - 1].to_vec();

        let target_module = match self.find_module_by_name(&module_name) {
            Some(existing_module) => existing_module,
            None => self.insert_new_module(&module_name),
        };

        target_module.add_file(module_path_queue, file);
    }
}

#[cfg(test)]
impl ReportTree {
    pub fn from_source_files(source_files: Vec<TestedFile>) -> Self {
        ReportTree {
            aggregated: AggregatedCoverage::default(),
            modules: vec![],
            source_files,
        }
    }

    pub fn from_modules(modules: Vec<TestedModule>) -> Self {
        ReportTree {
            aggregated: AggregatedCoverage::default(),
            modules,
            source_files: vec![],
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        aggregation::{aggregated::assert_aggregated_counters_eq, with_path::WithPath},
        test_utils::builders::{
            generate_2_lines_1_covered_section, generate_3_lines_2_covered_section, FromStr,
            InsertSection,
        },
    };

    use super::{
        super::{tested_file::TestedFile, tested_module::TestedModule},
        ReportTree,
    };
    use lcov::report::{section::Key as SectionKey, Report as LcovReport};

    #[test]
    fn when_building_tree_with_an_empty_report_it_should_get_an_empty_report() {
        let original_report = LcovReport::new();
        let report_tree = ReportTree::from_original_report(original_report);
        assert_eq!(ReportTree::default(), report_tree);
    }

    #[test]
    fn when_building_tree_with_a_report_with_one_file_it_should_get_a_report_with_tested_file_child(
    ) {
        let original_report =
            LcovReport::new().insert_empty_section(SectionKey::from_str("main.cpp"));

        let report_tree = ReportTree::from_original_report(original_report);

        let tested_file = TestedFile::new("main.cpp", "main.cpp");
        let expected_tree = ReportTree::from_source_files(vec![tested_file]);

        assert_eq!(expected_tree, report_tree);
    }

    #[test]
    fn when_building_tree_with_a_report_with_one_file_nested_it_should_get_a_report_with_module_node(
    ) {
        let original_report =
            LcovReport::new().insert_empty_section(SectionKey::from_str("package/main.cpp"));

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
        let original_report = LcovReport::new()
            .insert_empty_section(SectionKey::from_str("package/sub-package/main.cpp"));

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
        let original_report = LcovReport::new()
            .insert_empty_section(SectionKey::from_str("main.cpp"))
            .insert_empty_section(SectionKey::from_str("module.cpp"));

        let report_tree = ReportTree::from_original_report(original_report);

        let tested_file_main = TestedFile::new("main.cpp", "main.cpp");
        let tested_file_module = TestedFile::new("module.cpp", "module.cpp");
        let expected_tree =
            ReportTree::from_source_files(vec![tested_file_main, tested_file_module]);

        assert_eq!(expected_tree, report_tree);
    }

    #[test]
    fn when_building_tree_with_report_2_files_deeply_nested_shall_have_2_files_in_report_tree() {
        let original_report = LcovReport::new()
            .insert_empty_section(SectionKey::from_str("my/package/main.cpp"))
            .insert_empty_section(SectionKey::from_str("my/package/module.cpp"));

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
        let original_report = LcovReport::new()
            .insert_empty_section(SectionKey::from_str("my/package/main.cpp"))
            .insert_empty_section(SectionKey::from_str("yours/module.cpp"));

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
        let original_report = LcovReport::new()
            .insert_empty_section(SectionKey::from_str("package/sub-package/main.cpp"));

        let report_tree = ReportTree::from_original_report(original_report);

        let package = report_tree.modules.get(0).unwrap();
        let sub_package = package.get_module_at(0);
        let file = sub_package.get_source_file_at(0);

        assert_eq!(file.get_path(), vec!["package", "sub-package", "main.cpp"]);
        assert_eq!(sub_package.get_path(), vec!["package", "sub-package"]);
        assert_eq!(package.get_path(), vec!["package"]);
    }

    #[test]
    fn when_building_tree_with_an_empty_report_it_should_get_0_aggregates() {
        let original_report = LcovReport::new();
        let report_tree = ReportTree::from_original_report(original_report);

        assert_aggregated_counters_eq(&report_tree.aggregated.lines, 0, 0);
        assert_aggregated_counters_eq(&report_tree.aggregated.functions, 0, 0);
        assert_aggregated_counters_eq(&report_tree.aggregated.branches, 0, 0);
    }

    #[test]
    fn when_building_tree_with_a_top_level_file_with_coverage_data_it_should_get_the_same_aggregate(
    ) {
        let original_report = LcovReport::new().insert_section(
            SectionKey::from_str("main.cpp"),
            generate_3_lines_2_covered_section(),
        );

        let report_tree = ReportTree::from_original_report(original_report);
        let tested_file = report_tree.source_files.get(0).unwrap();

        assert_aggregated_counters_eq(&tested_file.aggregated.lines, 3, 2);
        assert_aggregated_counters_eq(&report_tree.aggregated.lines, 3, 2);
    }

    #[test]
    fn when_building_tree_with_a_nested_file_with_coverage_data_it_should_get_the_aggregate_all_along(
    ) {
        let original_report = LcovReport::new()
            .insert_section(
                SectionKey::from_str("module1/sub-module/main.cpp"),
                generate_3_lines_2_covered_section(),
            )
            .insert_section(
                SectionKey::from_str("module2/main.cpp"),
                generate_2_lines_1_covered_section(),
            );

        // Aggregate
        let report_tree = ReportTree::from_original_report(original_report);
        let module1 = report_tree.modules.get(0).unwrap();
        let sub_module1 = module1.get_module_at(0);
        let module2 = report_tree.modules.get(1).unwrap();

        assert_aggregated_counters_eq(&report_tree.aggregated.lines, 5, 3);
        assert_aggregated_counters_eq(&module1.aggregated.lines, 3, 2);
        assert_aggregated_counters_eq(&sub_module1.aggregated.lines, 3, 2);
        assert_aggregated_counters_eq(&sub_module1.get_source_file_at(0).aggregated.lines, 3, 2);
        assert_aggregated_counters_eq(&module2.aggregated.lines, 2, 1);
        assert_aggregated_counters_eq(&module2.get_source_file_at(0).aggregated.lines, 2, 1);
    }
}
