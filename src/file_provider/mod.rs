use std::path::PathBuf;

use crate::models::file_lines_provider::FileLinesProvider;

pub struct LocalFileLinesProvider {
    file_path: PathBuf,
}

impl LocalFileLinesProvider {
    pub fn new(file_path: PathBuf) -> Self {
        LocalFileLinesProvider { file_path }
    }
}

impl FileLinesProvider for LocalFileLinesProvider {
    fn get_file_lines(&self) -> Result<Vec<String>, std::io::Error> {
        let file = std::fs::read_to_string(&self.file_path)?;
        let lines: Vec<String> = file.lines().map(|s| s.to_string()).collect();
        Ok(lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_file_absolute_line_1() {
        let file = LocalFileLinesProvider::new(PathBuf::from("tests/fixtures/my_code.cpp"));
        let lines = file.get_file_lines().unwrap();
        assert_eq!(
            lines, 
            vec![
                "#include <iostream>",
                "", 
                "unsigned long long factorial(int n) {", 
                "    if (n == 0)", 
                "        return 1;", 
                "    else", 
                "        return n * factorial(n - 1);", 
                "}", 
                "", 
                "int main() {", 
                "    int number;", 
                "    std::cout << \"Enter a positive integer: \";", 
                "    std::cin >> number;", 
                "    std::cout << \"Factorial of \" << number << \" = \" << factorial(number);", 
                "    std::cout << std::endl;", 
                "    return 0;", "}"
            ]
        );
    }
}
