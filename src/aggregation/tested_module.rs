use crate::core::{AggregatedCoverage, TestedContainer, TestedFile};

use super::{tested_file::TestedCodeFile, with_path::WithPath};

#[derive(Debug, PartialEq, Default)]
pub struct TestedModule {
    name: String,
    path: String,
    source_files: Vec<TestedCodeFile>,
    modules: Vec<TestedModule>,
    aggregated: AggregatedCoverage,
}

impl TestedModule {
    pub fn new(path: String, name: String) -> Self {
        TestedModule {
            name,
            path,
            aggregated: AggregatedCoverage::default(),
            ..Default::default()
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn add_file(&mut self, path: Vec<String>, file: TestedCodeFile) {
        self.aggregated.add(&file.get_aggregated_coverage());

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

    pub fn enumerate_code_files(&self) -> impl Iterator<Item = &TestedCodeFile> {
        self.source_files
            .iter()
            .chain(self.modules.iter().flat_map(|m| m.enumerate_code_files()))
            .collect::<Vec<&TestedCodeFile>>()
            .into_iter()
    }
}

impl TestedContainer for TestedModule {
    fn get_name(&self) -> &str {
        &self.name
    }

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

impl WithPath for TestedModule {
    fn get_path_string(&self) -> String {
        self.path.clone()
    }
}

#[cfg(test)]
impl TestedModule {
    pub fn from_source_files(path: &str, name: &str, source_files: Vec<TestedCodeFile>) -> Self {
        let mut module = TestedModule::new(String::from(path), String::from(name));
        for source_file in source_files {
            module.add_file(vec![], source_file);
        }
        module
    }

    pub fn from_modules(path: &str, name: &str, modules: Vec<TestedModule>) -> Self {
        let mut module = TestedModule::new(String::from(path), String::from(name));
        for submodule in modules {
            module.modules.push(submodule);
        }
        module
    }

    pub fn get_module_at(&self, i: usize) -> &TestedModule {
        self.modules.get(i).unwrap()
    }

    pub fn get_source_file_at(&self, i: usize) -> &TestedCodeFile {
        self.source_files.get(i).unwrap()
    }
}

#[cfg(test)]
mod tests {

    use crate::aggregation::{
        aggregated::assert_aggregated_counters_eq, fixtures::AggregatedFixtures,
    };

    use super::*;

    #[test]
    fn when_creating_a_tested_module_the_name_and_path_should_be_set() {
        let tested_module = TestedModule::new("path/to/name".to_string(), "name".to_string());
        assert_eq!(tested_module.name, "name");
        assert_eq!(tested_module.path, "path/to/name");
        assert_eq!(tested_module.get_name(), "name");
    }

    #[test]
    fn when_adding_a_file_with_an_empty_path_it_should_add_it_directly_to_sources() {
        let mut tested_module = TestedModule::new("section".to_string(), "name".to_string());
        let tested_file = TestedCodeFile::new("section/file.cpp", "file.cpp");
        tested_module.add_file(vec![], tested_file);

        assert!(tested_module.modules.is_empty());
        assert_eq!(tested_module.source_files.len(), 1);
        assert_eq!(
            tested_module.get_source_file_at(0).get_path_string(),
            "section/file.cpp"
        );
    }

    #[test]
    fn when_adding_a_file_with_a_1_level_path_it_should_add_it_inside_a_module() {
        let mut tested_module = TestedModule::new("section".to_string(), "name".to_string());
        let tested_file = TestedCodeFile::new("section/submodule/file.cpp", "file.cpp");
        tested_module.add_file(vec!["submodule".to_string()], tested_file);

        assert!(tested_module.source_files.is_empty());

        assert_eq!(tested_module.modules.len(), 1);
        let module = tested_module.get_module_at(0);
        assert_eq!(module.path, "section/submodule");

        assert_eq!(module.source_files.len(), 1);
        let source_file = module.get_source_file_at(0);
        assert_eq!(source_file.get_path_string(), "section/submodule/file.cpp");
    }

    #[test]
    fn when_adding_2_file_to_the_same_sub_module_it_should_have_2_files_inside_submodule() {
        let mut tested_module = TestedModule::new("section".to_string(), "name".to_string());
        let tested_file = TestedCodeFile::new("section/submodule/file.cpp", "file.cpp");
        tested_module.add_file(vec!["submodule".to_string()], tested_file);
        let tested_file = TestedCodeFile::new("section/submodule/file2.cpp", "file2.cpp");
        tested_module.add_file(vec!["submodule".to_string()], tested_file);

        assert!(tested_module.source_files.is_empty());

        assert_eq!(tested_module.modules.len(), 1);
        let module = tested_module.get_module_at(0);
        assert_eq!(module.path, "section/submodule");

        assert_eq!(module.source_files.len(), 2);
        let source_file = module.get_source_file_at(0);
        assert_eq!(source_file.get_path_string(), "section/submodule/file.cpp");
        let source_file = module.get_source_file_at(1);
        assert_eq!(source_file.get_path_string(), "section/submodule/file2.cpp");
    }

    #[test]
    fn when_creating_empty_module_it_should_have_aggregate_0() {
        let tested_module = TestedModule::new("section".to_string(), "name".to_string());

        assert_aggregated_counters_eq(&tested_module.aggregated.lines, 0, 0);
        assert_aggregated_counters_eq(&tested_module.aggregated.functions, 0, 0);
        assert_aggregated_counters_eq(&tested_module.aggregated.branches, 0, 0);
    }

    #[test]
    fn when_creating_a_tested_module_with_empty_file_it_should_have_aggregate_0() {
        let tested_file = TestedCodeFile::new("section/file.cpp", "file.cpp");
        let tested_module = TestedModule::from_source_files("section", "name", vec![tested_file]);

        assert_aggregated_counters_eq(&tested_module.aggregated.lines, 0, 0);
        assert_aggregated_counters_eq(&tested_module.aggregated.functions, 0, 0);
        assert_aggregated_counters_eq(&tested_module.aggregated.branches, 0, 0);
    }

    #[test]
    fn when_creating_a_tested_module_with_one_file_it_should_get_same_aggregate() {
        let aggregated = AggregatedFixtures::get_file_aggregates_10_5();
        let tested_file =
            TestedCodeFile::with_aggregated("section/file.cpp", "file.cpp", aggregated);
        let mut tested_module = TestedModule::new("section".to_string(), "name".to_string());
        tested_module.add_file(vec![], tested_file);

        assert_aggregated_counters_eq(&tested_module.aggregated.lines, 10, 5);
    }

    #[test]
    fn when_creating_a_tested_module_with_two_file_it_should_get_same_aggregate() {
        // Create 2 files and add them to the module
        let tested_file1 = TestedCodeFile::with_aggregated(
            "section/file.cpp",
            "file.cpp",
            AggregatedFixtures::get_file_aggregates_10_5(),
        );
        let tested_file2 = TestedCodeFile::with_aggregated(
            "section/module/file2.cpp",
            "file2.cpp",
            AggregatedFixtures::get_file_aggregates_3_1(),
        );
        let mut tested_module = TestedModule::new("section".to_string(), "name".to_string());
        tested_module.add_file(vec![], tested_file1);
        tested_module.add_file(vec!["module".to_string()], tested_file2);

        // Check the aggregated values of the top module
        let module = tested_module.get_module_at(0);
        assert_aggregated_counters_eq(&tested_module.aggregated.lines, 13, 6);
        assert_aggregated_counters_eq(&module.aggregated.lines, 3, 1);
    }
}
