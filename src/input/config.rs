use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Input {
    pub name: Option<String>,
    pub prefix: Option<PathBuf>,
    pub path: PathBuf,
}
impl Input {
    pub fn from_path(path: PathBuf) -> Self {
        Self {
            path,
            ..Default::default()
        }
    }

    pub fn from_name_and_path(name: String, path: PathBuf) -> Self {
        Self {
            name: Some(name),
            path,
            ..Default::default()
        }
    }

    pub fn from_name_prefix_and_path(name: String, prefix: PathBuf, path: PathBuf) -> Self {
        Self {
            name: Some(name),
            prefix: Some(prefix),
            path,
        }
    }
}

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub inputs: Vec<Input>,
    pub output: PathBuf,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_config_from_toml() {
        let config = toml::from_str::<Config>(
            r#"name = "test"
output = "test"

[[inputs]]
name = "test1"
prefix = "test1"
path = "test1"

[[inputs]]
name = "test2"
path = "test2"

[[inputs]]
path = "test3"
"#,
        );

        assert_eq!(
            config.unwrap(),
            Config {
                name: "test".to_string(),
                inputs: vec![
                    Input {
                        name: Some("test1".to_string()),
                        prefix: Some(PathBuf::from("test1")),
                        path: PathBuf::from("test1"),
                    },
                    Input {
                        name: Some("test2".to_string()),
                        path: PathBuf::from("test2"),
                        ..Default::default()
                    },
                    Input {
                        path: PathBuf::from("test3"),
                        ..Default::default()
                    },
                ],
                output: PathBuf::from("test"),
            }
        );
    }
}
