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
