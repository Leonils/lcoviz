trait ToHtml {
    fn to_html(&self) -> String;
}

struct Div {
    class_names: Vec<String>,
    children: Vec<Box<dyn ToHtml>>,
}
impl Div {
    fn new() -> Self {
        Div {
            class_names: Vec::new(),
            children: Vec::new(),
        }
    }
    fn with_class(mut self, class: &str) -> Self {
        self.class_names.push(class.to_string());
        self
    }
    fn with_child(mut self, child: impl ToHtml + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }
}
impl ToHtml for Div {
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
}
