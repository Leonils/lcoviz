use super::html_builder::HtmlNode;

pub trait ComponentsFactory {
    fn create_line(&self, line_number: u32, count_number: u64) -> HtmlNode;
}
