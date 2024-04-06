use super::html_builder::HtmlNode;

pub trait ComponentsFactory {
    fn create_header(&self, title: &str) -> HtmlNode;
    fn create_code(&self, lines: Vec<HtmlNode>) -> HtmlNode;
    fn create_line(&self, line_number: u32, count_number: u64, line_content: String) -> HtmlNode;
}
