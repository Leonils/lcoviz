use crate::input::config::{Config, Input};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Printer {
    fn println(&self, message: &str);
}

pub struct Console;
impl Printer for Console {
    fn println(&self, message: &str) {
        println!("{}", message);
    }
}

pub struct CliOutput<TConsole: Printer> {
    console: TConsole,
}
impl<TConsole: Printer> CliOutput<TConsole> {
    pub fn new(console: TConsole) -> Self {
        CliOutput { console }
    }

    fn print_status(&self, title: &str, status: &str) {
        const BOLD: &str = "\x1b[1m";
        const GREEN: &str = "\x1b[32m";
        const RESET: &str = "\x1b[0m";
        let message = format!("{}{}{: >12} {}{}", BOLD, GREEN, title, RESET, status);
        self.console.println(&message);
    }

    pub fn print_error(&self, error: &str) {
        const BOLD: &str = "\x1b[1m";
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";
        let message = format!("{}{}{: >12} {}{}", BOLD, RED, "Error", RESET, error);
        self.console.println(&message);
    }

    fn print_input(&self, input: &Input) {
        self.print_status(
            "",
            &format!(
                "  - {}{}",
                input
                    .name
                    .as_ref()
                    .map_or("".to_string(), |f| format!("{}: ", f)),
                input.path.display()
            ),
        );
    }

    pub fn print_introduction(&self, config: &Config) {
        self.print_status(
            "Generating",
            &format!(
                "HTML report for {} input(s) lcov files",
                config.inputs.len()
            ),
        );

        self.print_status("", &format!("Report name: '{}'", config.name));
        self.print_status("", &format!("Reporter: '{}'", config.reporter.to_str()));
        self.print_status("", "Inputs: ");
        for input in config.inputs.iter() {
            self.print_input(input);
        }
    }

    pub fn print_conclusion(&self, output: &str) {
        self.print_status("Success", &format!("Report generated at {}", output));
    }

    pub fn print_help(&self) {
        self.console.println(include_str!("help.txt"));
    }

    pub fn print_command_help(&self, command: &str) {
        match command {
            "report" => self.console.println(include_str!("help.report.txt")),
            "to-file" => self.console.println(include_str!("help.to-file.txt")),
            "from-file" => self.console.println(include_str!("help.from-file.txt")),
            _ => self.console.println("Unknown command"),
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::input::config::Reporter;

    use super::*;

    impl MockPrinter {
        fn expect(mut self, m: &'static str) -> Self {
            self.expect_println()
                .times(1)
                .withf(move |message| {
                    let message = message.replace("\x1b[1m", "");
                    let message = message.replace("\x1b[32m", "");
                    let message = message.replace("\x1b[31m", "");
                    let message = message.replace("\x1b[0m", "");
                    let message = message.trim();
                    return message == m;
                })
                .return_const(());
            self
        }
    }

    #[test]
    fn test_print_introduction() {
        let console = MockPrinter::new()
            .expect("Generating HTML report for 2 input(s) lcov files")
            .expect("Report name: 'test'")
            .expect("Reporter: 'html-full-light'")
            .expect("Inputs:")
            .expect("- test1: test1/test1")
            .expect("- test2");

        CliOutput::new(console).print_introduction(&Config {
            name: "test".to_string(),
            inputs: vec![
                Input::from_name_and_path("test1".to_string(), PathBuf::from("test1/test1")),
                Input::from_path(PathBuf::from("test2")),
            ],
            output: PathBuf::from("test"),
            reporter: Reporter::default(),
        });
    }

    #[test]
    fn test_print_conclusion() {
        let console = MockPrinter::new().expect("Success Report generated at test");
        CliOutput::new(console).print_conclusion("test");
    }

    #[test]
    fn text_print_error() {
        let console = MockPrinter::new().expect("Error test");
        CliOutput::new(console).print_error("test");
    }
}
