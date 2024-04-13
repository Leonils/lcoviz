pub trait ToHtml {
    fn to_html(&self) -> String;
}

pub struct Text {
    level: u8,
    content: String,
}
impl Text {
    pub fn new(content: &str) -> Self {
        Text {
            level: 0,
            content: content.to_string(),
        }
    }

    pub fn h1(content: &str) -> Self {
        Text {
            level: 1,
            content: content.to_string(),
        }
    }

    pub fn h2(content: &str) -> Self {
        Text {
            level: 2,
            content: content.to_string(),
        }
    }
}
impl ToHtml for Text {
    fn to_html(&self) -> String {
        match self.level {
            0 => self.content.clone(),
            1 => format!("<h1>{}</h1>", self.content),
            2 => format!("<h2>{}</h2>", self.content),
            _ => panic!("Unsupported level: {}", self.level),
        }
    }
}

pub struct Div<'a> {
    class_names: Vec<String>,
    children: Vec<Box<dyn ToHtml + 'a>>,
}
impl<'a> Div<'a> {
    pub fn new() -> Self {
        Div {
            class_names: Vec::new(),
            children: Vec::new(),
        }
    }
    pub fn with_class(mut self, class: &str) -> Self {
        self.class_names.push(class.to_string());
        self
    }
    pub fn with_child(mut self, child: impl ToHtml + 'a) -> Self {
        self.children.push(Box::new(child));
        self
    }
    pub fn with_children(mut self, children: impl Iterator<Item = Box<dyn ToHtml + 'a>>) -> Self {
        for child in children {
            self.children.push(child);
        }
        self
    }
}
impl<'a> ToHtml for Div<'a> {
    fn to_html(&self) -> String {
        let class_attr = match self.class_names.len() {
            0 => String::new(),
            _ => format!(" class=\"{}\"", self.class_names.join(" ")),
        };
        let children_html: String = self.children.iter().map(|c| c.to_html()).collect();

        match children_html.len() {
            0 => format!("<div{} />", class_attr),
            _ => format!("<div{}>{}</div>", class_attr, children_html),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_div_shall_render() {
        let div = Div::new();
        assert_eq!(div.to_html(), "<div />");
    }

    #[test]
    fn empty_div_with_class_shall_render() {
        let div = Div::new().with_class("my-class");
        assert_eq!(div.to_html(), "<div class=\"my-class\" />");
    }

    #[test]
    fn empty_div_with_two_classes_shall_render() {
        let div = Div::new()
            .with_class("my-class")
            .with_class("my-other-class");
        assert_eq!(div.to_html(), "<div class=\"my-class my-other-class\" />");
    }

    #[test]
    fn div_with_one_child_shall_render() {
        let div = Div::new()
            .with_class("my-class")
            .with_child(Div::new().with_class("child-class"));
        assert_eq!(
            div.to_html(),
            "<div class=\"my-class\"><div class=\"child-class\" /></div>"
        );
    }

    #[test]
    fn div_with_two_children_shall_render() {
        let div = Div::new()
            .with_class("my-class")
            .with_child(Div::new().with_class("child1-class"))
            .with_child(Div::new().with_class("child2-class"));
        assert_eq!(
            div.to_html(),
            "<div class=\"my-class\"><div class=\"child1-class\" /><div class=\"child2-class\" /></div>"
        );
    }

    #[test]
    fn text_shall_render() {
        let text = Text::new("Hello, World!");
        assert_eq!(text.to_html(), "Hello, World!");
    }

    #[test]
    fn h1_shall_render() {
        let text = Text::h1("Hello, World!");
        assert_eq!(text.to_html(), "<h1>Hello, World!</h1>");
    }

    #[test]
    fn h2_shall_render() {
        let text = Text::h2("Hello, World!");
        assert_eq!(text.to_html(), "<h2>Hello, World!</h2>");
    }

    #[test]
    fn div_with_text_shall_render() {
        let div = Div::new()
            .with_class("my-class")
            .with_child(Text::new("Hello, World!"));
        assert_eq!(div.to_html(), "<div class=\"my-class\">Hello, World!</div>");
    }

    #[test]
    fn div_with_children_iter_shall_render() {
        let binding = vec![0, 1, 2];
        let children = binding
            .iter()
            .map(|i| Div::new().with_child(Text::new(&format!("c{}", i))))
            .map(|d| Box::new(d) as Box<dyn ToHtml>);

        let div = Div::new().with_class("my-class").with_children(children);
        assert_eq!(
            div.to_html(),
            "<div class=\"my-class\"><div>c0</div><div>c1</div><div>c2</div></div>"
        );
    }
}
