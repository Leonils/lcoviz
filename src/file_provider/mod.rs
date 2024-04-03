pub trait FileLinesProvider {
    fn get_file_lines(&self, start_line: usize, end_line: usize) -> Result<String, std::io::Error>;
}

pub struct LocalFileLinesProvider {
    file_path: String,
}

impl LocalFileLinesProvider {
    pub fn new(file_path: String) -> Self {
        LocalFileLinesProvider { file_path }
    }
}

impl FileLinesProvider for LocalFileLinesProvider {
    fn get_file_lines(&self, start_line: usize, end_line: usize) -> Result<String, std::io::Error> {
        let file = std::fs::read_to_string(&self.file_path)?;
        let lines: Vec<&str> = file.lines().collect();

        if start_line > lines.len() || end_line > lines.len() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "index out of bounds",
            ));
        }

        let result = lines
            .iter()
            .skip(start_line - 1)
            .take(end_line - start_line + 1)
            .map(|line| line.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_file_absolute_line_1() {
        let file = LocalFileLinesProvider::new("tests/fixtures/my_code.cpp".to_string());
        let line_1 = file.get_file_lines(1, 1).unwrap();
        assert_eq!(line_1, "#include <iostream>");
    }

    #[test]
    fn test_open_file_absolute_line_3_to_8() {
        let file = LocalFileLinesProvider::new("tests/fixtures/my_code.cpp".to_string());
        let line_3_to_8 = file.get_file_lines(3, 8).unwrap();
        assert_eq!(
            line_3_to_8,
            "unsigned long long factorial(int n) {
    if (n == 0)
        return 1;
    else
        return n * factorial(n - 1);
}"
        );
    }

    #[test]
    fn test_open_non_existent_file() {
        let file = LocalFileLinesProvider::new("tests/fixtures/non_existent_file.cpp".to_string());
        let result = file.get_file_lines(1, 1);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No such file or directory"));
    }

    #[test]
    fn test_open_out_of_bounds_lines() {
        let file = LocalFileLinesProvider::new("tests/fixtures/my_code.cpp".to_string());
        let result = file.get_file_lines(1, 100);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("index out of bounds"));
    }
}
