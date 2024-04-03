pub trait FileLinesProvider {
    fn get_file_lines(&self) -> Result<Vec<String>, std::io::Error>;
}
