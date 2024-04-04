use super::super::models::{
    components::ComponentsFactory, file_lines_provider::FileLinesProvider, html_builder::HtmlNode,
};

// Mock file provider
pub struct MockFilesProvider {
    nb_lines: u32,
}
impl MockFilesProvider {
    pub fn new(number_lines: u32) -> Self {
        MockFilesProvider {
            nb_lines: number_lines,
        }
    }
}
impl FileLinesProvider for MockFilesProvider {
    fn get_file_lines(&self) -> Result<Vec<String>, std::io::Error> {
        let mut lines: Vec<String> = Vec::with_capacity(self.nb_lines as usize);
        for i in 1..self.nb_lines + 1 {
            lines.push(format!("line_{}", i));
        }
        Ok(lines)
    }
}

// Mock components factory
pub struct MockComponentsFactory;
impl ComponentsFactory for MockComponentsFactory {
    fn create_code(&self, lines: Vec<HtmlNode>) -> HtmlNode {
        let mut container = HtmlNode::div();
        for line in lines {
            container.add_child(line);
        }
        container
    }

    fn create_line(&self, line_number: u32, count_number: u64, line_content: String) -> HtmlNode {
        HtmlNode::text(
            format!("line({}, {}, {});", line_number, count_number, line_content).as_str(),
        )
    }
}
