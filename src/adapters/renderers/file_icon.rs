use crate::core::TestedFile;

pub struct FileIcon;
impl FileIcon {
    pub fn get_icon_key(file: &impl TestedFile) -> Option<&str> {
        match file
            .get_path()
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
        {
            "rs" => Some("rust.svg"),
            "dart" => Some("dart.svg"),
            _ => None,
        }
    }

    pub fn get_resources_required_by_file(
        file: &impl TestedFile,
    ) -> Option<(&'static str, &'static str)> {
        match file
            .get_path()
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
        {
            "rs" => Some(("rust.svg", include_str!("resources/rust.svg"))),
            "dart" => Some(("dart.svg", include_str!("resources/dart.svg"))),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::WithPath;

    use super::*;

    struct MockTestFile {
        path: String,
    }
    impl MockTestFile {
        fn new(path: &str) -> Self {
            Self {
                path: path.to_string(),
            }
        }
    }
    impl WithPath for MockTestFile {
        fn get_name(&self) -> &str {
            unimplemented!()
        }
        fn get_path_string(&self) -> String {
            self.path.clone()
        }
        fn is_dir(&self) -> bool {
            false
        }
    }
    impl TestedFile for MockTestFile {
        fn get_aggregated_coverage(&self) -> &crate::core::AggregatedCoverage {
            unimplemented!()
        }
        fn get_functions(&self) -> impl Iterator<Item = (String, u64)> {
            return [].iter().cloned();
        }
        fn get_line_coverage(&self, _line: u32) -> Option<u64> {
            unimplemented!()
        }
        fn get_original_file_path(&self) -> std::path::PathBuf {
            unimplemented!()
        }
    }

    #[test]
    fn get_icon_key_should_return_rust_icon_key() {
        let file = MockTestFile::new("src/main.rs");
        assert_eq!(FileIcon::get_icon_key(&file), Some("rust.svg"));
    }

    #[test]
    fn get_icon_key_should_return_dart_icon_key() {
        let file = MockTestFile::new("src/main.dart");
        assert_eq!(FileIcon::get_icon_key(&file), Some("dart.svg"));
    }

    #[test]
    fn get_icon_key_should_return_none() {
        let file = MockTestFile::new("src/main.unknown");
        assert_eq!(FileIcon::get_icon_key(&file), None);
    }

    #[test]
    fn get_resources_required_by_file_should_return_rust_resources() {
        let file = MockTestFile::new("src/main.rs");
        assert_eq!(
            FileIcon::get_resources_required_by_file(&file),
            Some(("rust.svg", include_str!("resources/rust.svg")))
        );
    }

    #[test]
    fn get_resources_required_by_file_should_return_dart_resources() {
        let file = MockTestFile::new("src/main.dart");
        assert_eq!(
            FileIcon::get_resources_required_by_file(&file),
            Some(("dart.svg", include_str!("resources/dart.svg")))
        );
    }

    #[test]
    fn get_resources_required_by_file_should_return_none() {
        let file = MockTestFile::new("src/main.js");
        assert_eq!(FileIcon::get_resources_required_by_file(&file), None);
    }
}
