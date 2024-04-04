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

#[cfg(test)]
mod tests {
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
        let tested_file = TestedFile::new("section/file.cpp", "file.cpp");
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
        let tested_file = TestedFile::new("section/submodule/file.cpp", "file.cpp");
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
        let tested_file = TestedFile::new("section/submodule/file.cpp", "file.cpp");
        tested_module.add_file(vec!["submodule".to_string()], tested_file);
        let tested_file = TestedFile::new("section/submodule/file2.cpp", "file2.cpp");
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
}
