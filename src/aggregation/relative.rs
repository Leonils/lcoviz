use std::path::PathBuf;

use pathdiff::diff_paths;

use crate::core::TestedFile;

use super::{
    tested_file::TestedCodeFile, tested_module::TestedModule, tested_root::TestedRoot,
    with_path::WithPath,
};

trait RelativePath {
    fn get_path_relative_to(&self, source: &PathBuf) -> PathBuf;
}

impl RelativePath for TestedCodeFile {
    fn get_path_relative_to(&self, source: &PathBuf) -> PathBuf {
        diff_paths(self.get_file_path(), source).unwrap()
    }
}

impl RelativePath for TestedModule {
    fn get_path_relative_to(&self, source: &PathBuf) -> PathBuf {
        diff_paths(self.get_path_string(), source).unwrap()
    }
}

impl RelativePath for TestedRoot {
    fn get_path_relative_to(&self, source: &PathBuf) -> PathBuf {
        diff_paths(self.get_prefix(), source).unwrap()
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::aggregation::{
        input::AggregatorInput, relative::RelativePath, tested_file::TestedCodeFile,
        tested_module::TestedModule, tested_root::TestedRoot, with_path::WithPath,
    };

    #[test]
    fn when_getting_path_relative_to_root_it_should_return_the_path() {
        let tested_file = TestedCodeFile::new("/path/file.cpp", "file.cpp");
        let path = PathBuf::from("/");
        assert_eq!(
            tested_file.get_path_relative_to(&path),
            PathBuf::from("path/file.cpp")
        );
    }

    #[test]
    fn when_getting_path_relative_to_prefix_it_should_return_the_path() {
        let tested_file = TestedCodeFile::new("/path/file.cpp", "file.cpp");
        let path = PathBuf::from("/path");
        assert_eq!(
            tested_file.get_path_relative_to(&path),
            PathBuf::from("file.cpp")
        );
    }

    #[test]
    fn when_getting_path_relative_to_brother_it_should_return_the_path() {
        let tested_file = TestedCodeFile::new("/path/main/file.cpp", "file.cpp");
        let path = PathBuf::from("/path/other/");
        assert_eq!(
            tested_file.get_path_relative_to(&path),
            PathBuf::from("../main/file.cpp")
        );
    }

    #[test]
    fn when_getting_path_of_module_relative_to_code_file_it_should_return_the_path() {
        let tested_file = TestedCodeFile::new("/path/file.cpp", "file.cpp");
        let tested_module = TestedModule::new("/path".to_string(), "mod".to_string());

        let tested_module_path = PathBuf::from(tested_module.get_path_string());
        assert_eq!(
            tested_file.get_path_relative_to(&tested_module_path),
            PathBuf::from("file.cpp")
        );

        let tested_file_path = PathBuf::from(tested_file.get_path_string());
        assert_eq!(
            tested_module.get_path_relative_to(&tested_file_path),
            PathBuf::from("..")
        );
    }

    #[test]
    fn when_getting_path_of_root_relative_to_code_file_it_should_return_the_path() {
        let tested_file = TestedCodeFile::new("/path/file.cpp", "file.cpp");
        let tested_root =
            TestedRoot::new(AggregatorInput::new(lcov::report::Report::new()).with_prefix("/path"));

        let tested_root_path = PathBuf::from(tested_root.get_prefix());
        assert_eq!(
            tested_file.get_path_relative_to(&tested_root_path),
            PathBuf::from("file.cpp")
        );

        let tested_file_path = PathBuf::from(tested_file.get_path_string());
        assert_eq!(
            tested_root.get_path_relative_to(&tested_file_path),
            PathBuf::from("..")
        );
    }
}
