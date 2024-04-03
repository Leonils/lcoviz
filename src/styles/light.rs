use crate::models::{components::ComponentsFactory, html_builder::HtmlNode};

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
            format!(
                "  <div style=\"display:flex;\">
    <div style=\"width:50px;min-width:50px\">{}</div>
    <div style=\"width:50px;min-width:50px\">{}</div>
    <div><pre style=\"margin:0\">{}</pre></div>
  </div>
",
                line_number,
                count_number,
                line_content.replace("<", "&lt;").replace(">", "&gt;")
            )
            .as_str(),
        )
    }
}
