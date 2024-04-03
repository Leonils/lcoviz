pub fn get_file_lines(
    file_path: &str,
    start_line: usize,
    end_line: usize,
) -> Result<String, std::io::Error> {
    let file = std::fs::read_to_string(file_path)?;
    let lines: Vec<&str> = file.lines().collect();
    let mut result = String::new();
    for line in lines
        .iter()
        .skip(start_line - 1)
        .take(end_line - start_line + 1)
    {
        result.push_str(line);
        result.push_str("\n");
    }
    result.pop(); // Remove the last newline character
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_file_absolute_line_1() {
        let line_1 = get_file_lines("tests/fixtures/my_code.cpp", 1, 1).unwrap();
        assert_eq!(line_1, "#include <iostream>");
    }

    #[test]
    fn test_open_file_absolute_line_3_to_8() {
        let line_3_to_8 = get_file_lines("tests/fixtures/my_code.cpp", 3, 8).unwrap();
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
        let result = get_file_lines("tests/fixtures/non_existent_file.cpp", 1, 1);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No such file or directory"));
    }
}
