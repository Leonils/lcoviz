use crate::models::{components::ComponentsFactory, html_builder::HtmlNode};

// Mock components factory
pub struct MockComponentsFactory;
impl ComponentsFactory for MockComponentsFactory {
    fn create_header(&self, title: &str) -> HtmlNode {
        let mut header = HtmlNode::new("h1");
        header.add_child(HtmlNode::text(title));
        return header;
    }

    fn create_code(&self, lines: Vec<HtmlNode>) -> HtmlNode {
        let mut container = HtmlNode::div();
        for line in lines {
            container.add_child(line);
        }
        container
    }

    fn create_line(&self, line_number: u32, count_number: u64, line_content: String) -> HtmlNode {
        let color = if count_number > 0 { "green" } else { "red" };
        let background_color = if count_number > 0 {
            "#CCFFCC"
        } else {
            "#FFCCCC"
        };
        HtmlNode::text(
            format!(
                "  <div style=\"display:flex;color:{};background-color:{}\">
    <div style=\"width:50px;min-width:50px\">{}</div>
    <div style=\"width:50px;min-width:50px\">{}</div>
    <div><pre style=\"margin:0\">{}</pre></div>
  </div>
",
                color,
                background_color,
                line_number,
                count_number,
                line_content.replace("<", "&lt;").replace(">", "&gt;")
            )
            .as_str(),
        )
    }
}
