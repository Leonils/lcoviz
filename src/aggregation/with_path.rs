use super::{
    multi_report::MultiReport, tested_file::TestedCodeFile, tested_module::TestedModule,
    tested_root::TestedRoot,
};
use crate::core::WithPath;

impl WithPath for TestedCodeFile {
    fn get_name(&self) -> &str {
        self.get_file_name()
    }

    fn is_dir(&self) -> bool {
        false
    }

    fn get_path_string(&self) -> String {
        self.get_path_relative_to_prefix().to_string()
    }
}

impl WithPath for MultiReport {
    fn get_name(&self) -> &str {
        "MultiReport"
    }

    fn get_path_string(&self) -> String {
        "".to_string()
    }

    fn is_dir(&self) -> bool {
        true
    }
}

impl WithPath for TestedModule {
    fn get_name(&self) -> &str {
        self.get_module_name()
    }

    fn is_dir(&self) -> bool {
        true
    }

    fn get_path_string(&self) -> String {
        self.get_module_path().to_string()
    }
}

impl WithPath for TestedRoot {
    fn get_name(&self) -> &str {
        self.get_root_name()
    }

    fn is_dir(&self) -> bool {
        true
    }

    fn get_path_string(&self) -> String {
        self.get_key().to_string()
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::{
        aggregation::{
            input::AggregatorInput, multi_report::MultiReport, tested_file::TestedCodeFile,
            tested_module::TestedModule, tested_root::TestedRoot,
        },
        core::WithPath,
    };

    mod path {
        use super::*;

        #[test]
        fn when_getting_path_of_file_it_should_return_the_path() {
            let tested_file = TestedCodeFile::new("path/file.cpp", "file.cpp");
            assert_eq!(tested_file.get_path_string(), "path/file.cpp");
        }

        #[test]
        fn when_getting_path_of_module_it_should_return_the_path() {
            let tested_module = TestedModule::new("path/mod".to_string(), "mod".to_string());
            assert_eq!(tested_module.get_path_string(), "path/mod");
        }

        #[test]
        fn when_getting_path_of_root_it_should_return_the_key() {
            let tested_root = TestedRoot::new(
                AggregatorInput::new(lcov::report::Report::new())
                    .with_prefix("path")
                    .with_key("key"),
            );
            assert_eq!(tested_root.get_path_string(), "key");
        }

        #[test]
        fn when_getting_path_of_default_root_it_shall_return_empty() {
            let default_root = TestedRoot::default();
            assert_eq!(default_root.get_path_string(), "");
        }

        #[test]
        fn when_getting_path_of_multi_report_it_shall_return_empty() {
            let multi_report = MultiReport::new();
            assert_eq!(multi_report.get_path_string(), "");
        }
    }

    mod relative_path {
        use super::*;

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

    mod is_dir {
        use super::*;

        #[test]
        fn test_files_are_not_directories() {
            let tested_file = TestedCodeFile::new("path/file.cpp", "file.cpp");
            assert!(!tested_file.is_dir());
        }

        #[test]
        fn modules_are_directories() {
            let tested_module = TestedModule::new("path".to_string(), "mod".to_string());
            assert!(tested_module.is_dir());
        }

        #[test]
        fn root_are_directories() {
            let tested_root = TestedRoot::default();
            assert!(tested_root.is_dir());
        }

        #[test]
        fn multi_report_are_directories() {
            let multi_report = MultiReport::new();
            assert!(multi_report.is_dir());
        }
    }

    mod names {
        use super::*;

        #[test]
        fn test_file_name_is_file_name() {
            let tested_file = TestedCodeFile::new("path/file.cpp", "file.cpp");
            assert_eq!(tested_file.get_name(), "file.cpp");
        }

        #[test]
        fn test_module_name_is_module_name() {
            let tested_module = TestedModule::new("path".to_string(), "mod".to_string());
            assert_eq!(tested_module.get_name(), "mod");
        }

        #[test]
        fn test_root_name_is_last_part_of_prefix() {
            let tested_root = TestedRoot::new(
                AggregatorInput::new(lcov::report::Report::new())
                    .with_prefix("my/prefix/to/project")
                    .with_key("key"),
            );
            assert_eq!(tested_root.get_name(), "project");
        }

        #[test]
        fn test_multi_report_name_is_multi_report() {
            let multi_report = MultiReport::new();
            assert_eq!(multi_report.get_name(), "MultiReport");
        }
    }
}
