use std::path::PathBuf;

#[derive(Debug, PartialEq)]
enum Input {
    LcovPath(PathBuf),
    WithName(String, PathBuf),
    WithPrefix(String, PathBuf, PathBuf),
}

#[derive(Debug, PartialEq, Default)]
pub struct Config {
    name: String,
    inputs: Vec<Input>,
}

#[derive(Debug, PartialEq, Default)]
pub struct CliConfigParser {
    args: Vec<String>,
    step: usize,
    name: Option<String>,
    inputs: Vec<Input>,
}
impl CliConfigParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse(mut self, args: &[String]) -> Result<Self, String> {
        self.args = args.to_vec();

        while let Some(arg) = self.next() {
            match arg.as_str() {
                "--name" => self.set_name()?,
                "--input" => self.add_input()?,
                _ => return Err(format!("Unknown argument: {}", arg)),
            }
        }

        Ok(self)
    }

    fn next(&mut self) -> Option<String> {
        if self.step < self.args.len() {
            let value = self.args[self.step].clone();
            self.step += 1;
            Some(value)
        } else {
            self.step += 1;
            None
        }
    }

    fn previous(&mut self) {
        if self.step > 0 {
            self.step -= 1;
        }
    }

    pub fn build(self) -> Config {
        Config {
            name: self.name.unwrap_or_else(|| "Test report".to_string()),
            inputs: self.inputs,
        }
    }

    fn get_next_value(&mut self, arg_name: &str) -> Result<String, String> {
        match self.next() {
            Some(value) if !value.starts_with("--") => Ok(value.clone()),
            _ => Err(format!("Argument {} requires a value", arg_name)),
        }
    }

    fn extract_input_args(&mut self) -> Result<Input, String> {
        let arg1 = self.get_next_value("--input")?;
        let arg2 = self.get_next_value("--input");
        if arg2.is_err() {
            self.previous();
            return Ok(Input::LcovPath(PathBuf::from(arg1)));
        }

        let arg3 = self.get_next_value("--input");
        if arg3.is_err() {
            self.previous();
            return Ok(Input::WithName(arg1, PathBuf::from(arg2.unwrap())));
        }

        Ok(Input::WithPrefix(
            arg1,
            PathBuf::from(arg2.unwrap()),
            PathBuf::from(arg3.unwrap()),
        ))
    }

    fn add_input(&mut self) -> Result<(), String> {
        let input = self.extract_input_args()?;
        self.inputs.push(input);
        Ok(())
    }

    fn set_name(&mut self) -> Result<(), String> {
        let name = self.get_next_value("--name")?;
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
                inputs: vec![Input::LcovPath(PathBuf::from("~/test.lcov"))]
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
                inputs: vec![
                    Input::LcovPath(PathBuf::from("~/test.lcov")),
                    Input::LcovPath(PathBuf::from("~/test2.lcov"))
                ]
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

    #[test]
    fn when_specifying_single_input_with_2_parts_it_shall_create_a_named_input() {
        assert_eq!(
            parse("--input named_root ~/test.lcov").unwrap().build(),
            Config {
                name: "Test report".to_string(),
                inputs: vec![Input::WithName(
                    "named_root".to_string(),
                    PathBuf::from("~/test.lcov")
                )]
            }
        );
    }

    #[test]
    fn when_specifying_single_input_with_3_parts_it_shall_create_a_named_input_with_prefix() {
        assert_eq!(
            parse("--input named_root /foo/bar ~/test.lcov")
                .unwrap()
                .build(),
            Config {
                name: "Test report".to_string(),
                inputs: vec![Input::WithPrefix(
                    "named_root".to_string(),
                    PathBuf::from("/foo/bar"),
                    PathBuf::from("~/test.lcov")
                )]
            }
        );
    }

    #[test]
    fn when_specifying_multiple_inputs_with_different_number_of_parts_it_shall_create_each_one_with_correct_variant(
    ) {
        assert_eq!(
            parse("--input named_root_1 ~/test.lcov --input ~/test2.lcov --input named_root_3 /foo/bar ~/test3.lcov")
                .unwrap()
                .build(),
            Config {
                name: "Test report".to_string(),
                inputs: vec![
                    Input::WithName("named_root_1".to_string(), PathBuf::from("~/test.lcov")),
                    Input::LcovPath(PathBuf::from("~/test2.lcov")),
                    Input::WithPrefix(
                        "named_root_3".to_string(),
                        PathBuf::from("/foo/bar"),
                        PathBuf::from("~/test3.lcov")
                    )
                ]
            }
        );
    }
}
