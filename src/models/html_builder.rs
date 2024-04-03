pub struct HtmlTag {
    tag: String,
    children: Vec<HtmlNode>,
}

impl HtmlTag {
    fn new(tag: &str) -> Self {
        HtmlTag {
            tag: tag.to_string(),
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: HtmlNode) {
        self.children.push(child);
    }
}

pub enum HtmlNode {
    Tag(HtmlTag),
    Text(String),
}

impl HtmlNode {
    pub fn div() -> Self {
        HtmlNode::new("div")
    }

    pub fn span() -> Self {
        HtmlNode::new("span")
    }

    pub fn strong() -> Self {
        HtmlNode::new("strong")
    }

    pub fn text(text: &str) -> Self {
        HtmlNode::Text(text.to_string())
    }

    pub fn new(tag: &str) -> Self {
        HtmlNode::Tag(HtmlTag::new(tag))
    }

    pub fn add_child(&mut self, child: HtmlNode) {
        match self {
            HtmlNode::Tag(tag) => tag.add_child(child),
            _ => panic!("Cannot add child to text node"),
        }
    }

    pub fn render(&self) -> String {
        match self {
            HtmlNode::Tag(tag) => {
                let children = tag
                    .children
                    .iter()
                    .map(|child| child.render())
                    .collect::<String>();
                format!("<{}>{}</{}>", tag.tag, children, tag.tag)
            }
            HtmlNode::Text(text) => text.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_div() {
        let div = HtmlNode::div();
        assert_eq!(div.render(), "<div></div>");
    }

    #[test]
    fn test_create_div_with_children() {
        let mut div = HtmlNode::div();
        let span = HtmlNode::span();
        div.add_child(span);
        assert_eq!(div.render(), "<div><span></span></div>");
    }

    #[test]
    fn test_create_nested_divs() {
        let mut div = HtmlNode::div();
        let mut span = HtmlNode::span();
        let strong = HtmlNode::strong();
        span.add_child(strong);
        div.add_child(span);
        assert_eq!(div.render(), "<div><span><strong></strong></span></div>");
    }

    #[test]
    fn test_create_div_with_text() {
        let mut div = HtmlNode::div();
        let text = HtmlNode::text("Hello, world!");
        div.add_child(text);
        assert_eq!(div.render(), "<div>Hello, world!</div>");
    }
}
