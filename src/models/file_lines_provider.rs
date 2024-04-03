pub trait FileLinesProvider {
    fn get_file_lines(&self, start_line: usize, end_line: usize) -> Result<String, std::io::Error>;
}
