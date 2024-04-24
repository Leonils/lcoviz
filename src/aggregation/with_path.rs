#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::{
        aggregation::{
            input::AggregatorInput, tested_file::TestedCodeFile, tested_module::TestedModule,
            tested_root::TestedRoot,
        },
        core::WithPath,
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
        let tested_file = TestedCodeFile::new("path/file.cpp", "file.cpp");
        let tested_root = TestedRoot::new(
            AggregatorInput::new(lcov::report::Report::new())
                .with_prefix("path")
                .with_key("path"),
        );

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
