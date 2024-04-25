#[derive(Debug, PartialEq)]
pub struct Config {
    name: String,
}

#[derive(Debug, PartialEq)]
pub struct CliConfigParser {
    name: Option<String>,
}
impl CliConfigParser {
    pub fn new() -> Self {
        Self { name: None }
    }

    pub fn parse(mut self, args: &[String]) -> Result<Self, String> {
        let mut args = args.iter();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--name" => self.set_name(args.next())?,
                _ => return Err(format!("Unknown argument: {}", arg)),
            }
        }

        Ok(self)
    }

    pub fn build(self) -> Config {
        Config {
            name: self.name.unwrap_or_else(|| "Test report".to_string()),
        }
    }

    fn set_name(&mut self, name: Option<&String>) -> Result<(), String> {
        if self.name.is_some() {
            return Err("Argument --name already provided".to_string());
        }
        if name.is_none() {
            return Err("Argument --name requires a value".to_string());
        }
        self.name = Some(name.unwrap().clone());
        Ok(())
    }
}

#[cfg(test)]
mod test {
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
                name: "test".to_string()
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
                name: "Test report".to_string()
            }
        );
    }
}
