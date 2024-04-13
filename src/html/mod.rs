trait ToHtml {
    fn to_html(&self) -> String;
}

struct Div {
    class_names: Vec<String>,
}
impl Div {
    fn new() -> Self {
        Div {
            class_names: Vec::new(),
        }
    }
    fn with_class(mut self, class: &str) -> Self {
        self.class_names.push(class.to_string());
        self
    }
}
impl ToHtml for Div {
    fn to_html(&self) -> String {
        let class_attr = match self.class_names.len() {
            0 => String::new(),
            _ => format!(" class=\"{}\"", self.class_names.join(" ")),
        };
        format!("<div{}></div>", class_attr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_div_shall_render() {
        let div = Div::new();
        assert_eq!(div.to_html(), "<div></div>");
    }

    #[test]
    fn empty_div_with_class_shall_render() {
        let div = Div::new().with_class("my-class");
        assert_eq!(div.to_html(), "<div class=\"my-class\"></div>");
    }

    #[test]
    fn empty_div_with_two_classes_shall_render() {
        let div = Div::new()
            .with_class("my-class")
            .with_class("my-other-class");
        assert_eq!(
            div.to_html(),
            "<div class=\"my-class my-other-class\"></div>"
        );
    }
}
