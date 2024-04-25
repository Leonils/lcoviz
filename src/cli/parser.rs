use std::path::PathBuf;

#[derive(Debug, PartialEq, Default)]
pub struct Config {
    name: String,
    inputs: Vec<PathBuf>,
}

#[derive(Debug, PartialEq, Default)]
pub struct CliConfigParser {
    name: Option<String>,
    inputs: Vec<PathBuf>,
}
impl CliConfigParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse(mut self, args: &[String]) -> Result<Self, String> {
        let mut args = args.iter();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--name" => self.set_name(Self::get_next_value("--name", &mut args)?)?,
                "--input" => self
                    .inputs
                    .push(PathBuf::from(Self::get_next_value("--input", &mut args)?)),
                _ => return Err(format!("Unknown argument: {}", arg)),
            }
        }

        Ok(self)
    }

    pub fn build(self) -> Config {
        Config {
            name: self.name.unwrap_or_else(|| "Test report".to_string()),
            inputs: self.inputs,
        }
    }

    fn get_next_value(
        arg_name: &str,
        args: &mut std::slice::Iter<String>,
    ) -> Result<String, String> {
        match args.next() {
            Some(value) if !value.starts_with("--") => Ok(value.clone()),
            _ => Err(format!("Argument {} requires a value", arg_name)),
        }
    }

    fn set_name(&mut self, name: String) -> Result<(), String> {
        if self.name.is_some() {
            return Err("Argument --name already provided".to_string());
        }
        self.name = Some(name);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::*;

    fn parse(args: &str) -> Result<CliConfigParser, String> {
        let args = args
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        CliConfigParser::new().parse(&args)
    }

    #[test]
    fn test_config() {
        assert_eq!(
            parse("--name test").unwrap().build(),
            Config {
                name: "test".to_string(),
                ..Default::default()
            }
        );
    }

    #[test]
    fn when_providing_2_names_it_shall_return_error() {
        assert_eq!(
            parse("--name test --name test2").unwrap_err(),
            "Argument --name already provided"
        );
    }

    #[test]
    fn when_providing_name_without_value_it_shall_return_error() {
        assert_eq!(
            parse("--name").unwrap_err(),
            "Argument --name requires a value"
        );
    }

    #[test]
    fn when_providing_unknown_argument_it_shall_return_error() {
        assert_eq!(
            parse("--unknown").unwrap_err(),
            "Unknown argument: --unknown"
        );
    }

    #[test]
    fn when_not_providing_name_it_shall_use_default() {
        assert_eq!(
            parse("").unwrap().build(),
            Config {
                name: "Test report".to_string(),
                ..Default::default()
            }
        );
    }

    #[test]
    fn when_specifying_input_with_single_path_it_shall_add_it_to_config() {
        assert_eq!(
            parse("--input ~/test.lcov").unwrap().build(),
            Config {
                name: "Test report".to_string(),
                inputs: vec![PathBuf::from("~/test.lcov")]
            }
        );
    }

    #[test]
    fn when_specifying_input_with_multiple_paths_it_shall_add_them_to_config() {
        assert_eq!(
            parse("--input ~/test.lcov --input ~/test2.lcov")
                .unwrap()
                .build(),
            Config {
                name: "Test report".to_string(),
                inputs: vec![PathBuf::from("~/test.lcov"), PathBuf::from("~/test2.lcov")]
            }
        );
    }

    #[test]
    fn when_specifying_input_without_value_it_shall_return_error() {
        assert_eq!(
            parse("--input").unwrap_err(),
            "Argument --input requires a value"
        );
    }

    #[test]
    fn another_arg_shall_not_count_as_a_value_for_name() {
        assert_eq!(
            parse("--name --input ~/test.lcov").unwrap_err(),
            "Argument --name requires a value"
        );
    }
}
