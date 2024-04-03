use super::models::{
    components::ComponentsFactory, file_lines_provider::FileLinesProvider, html_builder::HtmlNode,
};

// Mock file provider
pub struct MockFilesProvider;
impl FileLinesProvider for MockFilesProvider {
    fn get_file_lines(&self, start_line: usize, end_line: usize) -> Result<String, std::io::Error> {
        let mut lines: Vec<String> = Vec::with_capacity(end_line - start_line + 1);
        for i in start_line..end_line {
            lines.push(format!("line_{}", i));
        }
        Ok(lines.join("\n"))
    }
}

// Mock components factory
pub struct MockComponentsFactory;
impl ComponentsFactory for MockComponentsFactory {
    fn create_line(&self, line_number: u32, count_number: u64, line_content: String) -> HtmlNode {
        HtmlNode::text(
            format!(
                "<>Line {}[{}]: {}</>",
                line_number, count_number, line_content
            )
            .as_str(),
        )
    }
}
