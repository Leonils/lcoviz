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
