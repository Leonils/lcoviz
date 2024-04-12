pub trait WithPath {
    fn get_path_string(&self) -> String;
    fn get_path(&self) -> Vec<String> {
        self.get_path_string()
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }
}
