use std::path::PathBuf;

use super::config::{Config, Input};

#[derive(Debug, PartialEq, Default)]
pub struct CliConfigParser {
    args: Vec<String>,
    step: usize,
    name: Option<String>,
    inputs: Vec<Input>,
    output: Option<PathBuf>,
}
impl CliConfigParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse(mut self, args: &[String]) -> Result<Self, String> {
        self.args = args.to_vec();

        while let Some(arg) = self.next() {
            let arg_str = arg.as_str();
            match arg_str {
                "--name" | "-n" => self.set_name(arg_str)?,
                "--input" | "-i" => self.add_input(arg_str)?,
                "--output" | "-o" => self.set_output(arg_str)?,
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

    pub fn build(self) -> Result<Config, String> {
        let output = self
            .output
            .ok_or_else(|| "Argument --output is required".to_string())?;

        Ok(Config {
            name: self.name.unwrap_or_else(|| "Test report".to_string()),
            inputs: self.inputs,
            output,
        })
    }

    fn get_next_value(&mut self, arg_name: &str) -> Result<String, String> {
        match self.next() {
            Some(value) if !value.starts_with("-") => Ok(value.clone()),
            _ => Err(format!("Argument {} requires a value", arg_name)),
        }
    }

    fn extract_input_args(&mut self, arg_name: &str) -> Result<Input, String> {
        let arg1 = self.get_next_value(arg_name)?;
        let arg2 = self.get_next_value(arg_name);
        if arg2.is_err() {
            self.previous();
            return Ok(Input {
                path: PathBuf::from(arg1),
                ..Default::default()
            });
        }

        let arg3 = self.get_next_value(arg_name);
        if arg3.is_err() {
            self.previous();
            return Ok(Input {
                name: Some(arg1.to_string()),
                path: PathBuf::from(arg2.unwrap()),
                ..Default::default()
            });
        }

        Ok(Input {
            name: Some(arg1.to_string()),
            prefix: Some(PathBuf::from(arg2.unwrap())),
            path: PathBuf::from(arg3.unwrap()),
        })
    }

    fn add_input(&mut self, arg_name: &str) -> Result<(), String> {
        let input = self.extract_input_args(arg_name)?;
        self.inputs.push(input);
        Ok(())
    }

    fn set_name(&mut self, arg_name: &str) -> Result<(), String> {
        let name = self.get_next_value(arg_name)?;
        if self.name.is_some() {
            return Err(format!("Argument {} already provided", arg_name));
        }
        self.name = Some(name);
        Ok(())
    }

    fn set_output(&mut self, arg_name: &str) -> Result<(), String> {
        let output = self.get_next_value(arg_name)?;
        if self.output.is_some() {
            return Err(format!("Argument {} already provided", arg_name));
        }
        self.output = Some(PathBuf::from(output));
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
    fn when_providing_output_only_it_shall_set_default_name() {
        assert_eq!(
            parse("--output output").unwrap().build().unwrap(),
            Config {
                output: PathBuf::from("output"),
                name: "Test report".to_string(),
                ..Default::default()
            }
        );
    }

    #[test]
    fn when_output_is_missing_it_shall_fail_to_build() {
        assert_eq!(
            parse("--name test").unwrap().build().unwrap_err(),
            "Argument --output is required"
        );
    }

    #[test]
    fn when_providing_name_and_output_it_shall_build_the_config() {
        assert_eq!(
            parse("--output output --name test")
                .unwrap()
                .build()
                .unwrap(),
            Config {
                output: PathBuf::from("output"),
                name: "test".to_string(),
                ..Default::default()
            }
        );
    }

    #[test]
    fn when_providing_output_without_value_it_shall_return_error() {
        assert_eq!(
            parse("--output").unwrap_err(),
            "Argument --output requires a value"
        );
    }

    #[test]
    fn when_providing_output_twice_it_shall_return_error() {
        assert_eq!(
            parse("--output output --output output2").unwrap_err(),
            "Argument --output already provided"
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
    fn when_specifying_input_with_single_path_it_shall_add_it_to_config() {
        assert_eq!(
            parse("--output output  --input ~/test.lcov")
                .unwrap()
                .build()
                .unwrap(),
            Config {
                output: PathBuf::from("output"),
                name: "Test report".to_string(),
                inputs: vec![Input::from_path(PathBuf::from("~/test.lcov"))]
            }
        );
    }

    #[test]
    fn when_specifying_input_with_multiple_paths_it_shall_add_them_to_config() {
        assert_eq!(
            parse("--output output --input ~/test.lcov --input ~/test2.lcov")
                .unwrap()
                .build()
                .unwrap(),
            Config {
                output: PathBuf::from("output"),
                name: "Test report".to_string(),
                inputs: vec![
                    Input::from_path(PathBuf::from("~/test.lcov")),
                    Input::from_path(PathBuf::from("~/test2.lcov"))
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
            parse("--output output --input named_root ~/test.lcov")
                .unwrap()
                .build()
                .unwrap(),
            Config {
                output: PathBuf::from("output"),
                name: "Test report".to_string(),
                inputs: vec![Input::from_name_and_path(
                    "named_root".to_string(),
                    PathBuf::from("~/test.lcov")
                )]
            }
        );
    }

    #[test]
    fn when_specifying_single_input_with_3_parts_it_shall_create_a_named_input_with_prefix() {
        assert_eq!(
            parse("--output output --input named_root /foo/bar ~/test.lcov")
                .unwrap()
                .build()
                .unwrap(),
            Config {
                output: PathBuf::from("output"),
                name: "Test report".to_string(),
                inputs: vec![Input::from_name_prefix_and_path(
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
            parse("--output output --input named_root_1 ~/test.lcov --input ~/test2.lcov --input named_root_3 /foo/bar ~/test3.lcov")
                .unwrap()
                .build().unwrap(),
            Config {
                output: PathBuf::from("output"),
                name: "Test report".to_string(),
                inputs: vec![
                    Input::from_name_and_path("named_root_1".to_string(), PathBuf::from("~/test.lcov")),
                    Input::from_path(PathBuf::from("~/test2.lcov")),
                    Input::from_name_prefix_and_path(
                        "named_root_3".to_string(),
                        PathBuf::from("/foo/bar"),
                        PathBuf::from("~/test3.lcov")
                    )
                ]
            }
        );
    }

    #[test]
    fn when_using_short_input_command_it_shall_parse_it_correctly() {
        assert_eq!(
            parse("-o output -i named_root_1 ~/test.lcov -i ~/test2.lcov -i named_root_3 /foo/bar ~/test3.lcov")
                .unwrap()
                .build().unwrap(),
            Config {
                output: PathBuf::from("output"),
                name: "Test report".to_string(),
                inputs: vec![
                    Input::from_name_and_path("named_root_1".to_string(), PathBuf::from("~/test.lcov")),
                    Input::from_path(PathBuf::from("~/test2.lcov")),
                    Input::from_name_prefix_and_path(
                        "named_root_3".to_string(),
                        PathBuf::from("/foo/bar"),
                        PathBuf::from("~/test3.lcov")
                    )
                ]
            }
        );
    }

    #[test]
    fn when_using_short_name_and_output_command_it_shall_parse_it_correctly() {
        assert_eq!(
            parse("-o output -n test").unwrap().build().unwrap(),
            Config {
                output: PathBuf::from("output"),
                name: "test".to_string(),
                ..Default::default()
            }
        );
    }

    #[test]
    fn when_using_short_commands_it_shall_detect_next_option_not_as_a_value() {
        assert_eq!(
            parse("-n -i ~/test.lcov").unwrap_err(),
            "Argument -n requires a value"
        );
    }

    #[test]
    fn path_of_input_shall_return_correct_path_for_each_variant() {
        assert_eq!(
            Input::from_path(PathBuf::from("path")).path,
            PathBuf::from("path")
        );
        assert_eq!(
            Input::from_name_and_path("name".to_string(), PathBuf::from("path")).path,
            PathBuf::from("path")
        );
        assert_eq!(
            Input::from_name_prefix_and_path(
                "name".to_string(),
                PathBuf::from("prefix"),
                PathBuf::from("path")
            )
            .path,
            PathBuf::from("path")
        );
    }
}
