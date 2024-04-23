use htmlescape::encode_minimal;

use crate::{core::LinkPayload, html::colors::get_percentage_class};

pub trait ToHtml {
    fn to_html(&self) -> String;
}

pub struct Text {
    level: u8,
    content: String,
    class_names: Vec<String>,
}
impl Text {
    pub fn new(content: &str) -> Self {
        Text {
            level: 0,
            content: content.to_string(),
            class_names: Vec::new(),
        }
    }

    pub fn h1(content: &str) -> Self {
        Text {
            level: 1,
            content: content.to_string(),
            class_names: Vec::new(),
        }
    }

    pub fn h2(content: &str) -> Self {
        Text {
            level: 2,
            content: content.to_string(),
            class_names: Vec::new(),
        }
    }

    pub fn with_class(mut self, class: &str) -> Self {
        self.class_names.push(class.to_string());
        self
    }
}
impl ToHtml for Text {
    fn to_html(&self) -> String {
        let class_attr = match self.class_names.len() {
            0 => String::new(),
            _ => format!(" class=\"{}\"", self.class_names.join(" ")),
        };
        match self.level {
            0 => encode_minimal(&self.content),
            i if i > 0 && i < 7 => format!(
                "<h{}{}>{}</h{}>",
                i,
                class_attr,
                encode_minimal(&self.content),
                i
            ),
            _ => panic!("Unsupported level: {}", self.level),
        }
    }
}

pub struct Link {
    href: String,
    child: Box<dyn ToHtml>,
}
impl Link {
    pub fn from_child(href: &str, child: Box<dyn ToHtml>) -> Self {
        Link {
            href: href.to_string(),
            child,
        }
    }
    pub fn from_text(href: &str, text: &str) -> Self {
        Link {
            href: href.to_string(),
            child: Box::new(Text::new(text)),
        }
    }
    pub fn from_link_payload(LinkPayload { link, text }: LinkPayload) -> Self {
        Link {
            href: link,
            child: Box::new(Text::new(&text)),
        }
    }
}
impl ToHtml for Link {
    fn to_html(&self) -> String {
        format!("<a href=\"{}\">{}</a>", self.href, self.child.to_html(),)
    }
}

pub struct Img {
    src: String,
    alt: String,
}
impl Img {
    pub fn new(src: &str, alt: &str) -> Self {
        Img {
            src: src.to_string(),
            alt: alt.to_string(),
        }
    }
}
impl ToHtml for Img {
    fn to_html(&self) -> String {
        format!(
            "<img src=\"{}\" alt=\"{}\" />",
            self.src,
            encode_minimal(&self.alt)
        )
    }
}

pub struct Pre {
    content: String,
}
impl Pre {
    pub fn new(content: &str) -> Self {
        Pre {
            content: content.to_string(),
        }
    }
}
impl ToHtml for Pre {
    fn to_html(&self) -> String {
        format!("<pre>{}</pre>", encode_minimal(&self.content))
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
    pub fn with_text(mut self, text: &str) -> Self {
        self.children.push(Box::new(Text::new(text)));
        self
    }
    pub fn with_children<Element: ToHtml + 'a>(
        mut self,
        children: impl Iterator<Item = Element>,
    ) -> Self {
        for child in children {
            self.children.push(Box::new(child));
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

        format!("<div{}>{}</div>", class_attr, children_html)
    }
}

pub struct Row<'a> {
    class_names: Vec<String>,
    cells: Vec<Box<dyn ToHtml + 'a>>,
}
impl<'a> Row<'a> {
    pub fn new() -> Self {
        Row {
            class_names: Vec::new(),
            cells: Vec::new(),
        }
    }
    pub fn with_class(mut self, class: &str) -> Self {
        self.class_names.push(class.to_string());
        self
    }
    pub fn with_cell(mut self, cell: impl ToHtml + 'a) -> Self {
        self.cells.push(Box::new(cell));
        self
    }
}
pub struct Table<'a> {
    rows: Vec<Row<'a>>,
}
impl<'a> Table<'a> {
    pub fn new() -> Self {
        Table { rows: Vec::new() }
    }
    pub fn with_row(mut self, row: Row<'a>) -> Self {
        self.rows.push(row);
        self
    }
    pub fn with_rows(mut self, rows: impl Iterator<Item = Row<'a>>) -> Self {
        for row in rows {
            self.rows.push(row);
        }
        self
    }
}
impl<'a> ToHtml for Row<'a> {
    fn to_html(&self) -> String {
        let class_attr = match self.class_names.len() {
            0 => String::new(),
            _ => format!(" class=\"{}\"", self.class_names.join(" ")),
        };
        let cells_html: String = self
            .cells
            .iter()
            .map(|c| format!("<td>{}</td>", c.to_html()))
            .collect();

        format!("<tr{}>{}</tr>", class_attr, cells_html)
    }
}
impl<'a> ToHtml for Table<'a> {
    fn to_html(&self) -> String {
        let rows_html: String = self.rows.iter().map(|r| r.to_html()).collect();

        format!("<table>{}</table>", rows_html)
    }
}

pub struct Gauge {
    percentage: Option<f32>,
    title: String,
}
impl Gauge {
    pub fn new(percentage: Option<f32>, title: &str) -> Self {
        Gauge {
            percentage,
            title: title.to_string(),
        }
    }
}
impl ToHtml for Gauge {
    fn to_html(&self) -> String {
        format!(
            r#"<div class="gauge"><div class="container"><div class="gauge-a"></div><div class="gauge-b"></div><div class="gauge-c {}" style="transform: rotate({:.2}turn)"></div><div class="gauge-data"><span class="percent">{}</h1></div></div><div>{}</div></div>"#,
            get_percentage_class("bg", &self.percentage),
            self.percentage.unwrap_or(0.0) / 200.,
            self.percentage
                .map_or("-".to_string(), |p| format!("{:.2}%", p)),
            self.title
        )
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

    #[test]
    fn div_with_one_child_shall_render() {
        let div = Div::new()
            .with_class("my-class")
            .with_child(Div::new().with_class("child-class"));
        assert_eq!(
            div.to_html(),
            "<div class=\"my-class\"><div class=\"child-class\"></div></div>"
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
            "<div class=\"my-class\"><div class=\"child1-class\"></div><div class=\"child2-class\"></div></div>"
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
    fn h2_with_class_names_shall_render() {
        let text = Text::h2("Hello, World!")
            .with_class("my-class")
            .with_class("my-other-class");
        assert_eq!(
            text.to_html(),
            "<h2 class=\"my-class my-other-class\">Hello, World!</h2>"
        );
    }

    #[test]
    fn h6_shall_render() {
        let text = Text {
            level: 6,
            content: "Hello, World!".to_string(),
            class_names: Vec::new(),
        };
        assert_eq!(text.to_html(), "<h6>Hello, World!</h6>");
    }

    #[test]
    #[should_panic]
    fn h7_shall_panic() {
        let text = Text {
            level: 7,
            content: "Hello, World!".to_string(),
            class_names: Vec::new(),
        };
        text.to_html();
    }

    #[test]
    fn div_with_text_shall_render() {
        let div = Div::new()
            .with_class("my-class")
            .with_child(Text::new("Hello, World!"));
        assert_eq!(div.to_html(), "<div class=\"my-class\">Hello, World!</div>");
    }

    #[test]
    fn div_with_direct_text_shall_render() {
        let div = Div::new().with_class("my-class").with_text("Hello, World!");
        assert_eq!(div.to_html(), "<div class=\"my-class\">Hello, World!</div>");
    }

    #[test]
    fn div_with_children_iter_shall_render() {
        let binding = vec![0, 1, 2];
        let children = binding
            .iter()
            .map(|i| Div::new().with_child(Text::new(&format!("c{}", i))));

        let div = Div::new().with_class("my-class").with_children(children);
        assert_eq!(
            div.to_html(),
            "<div class=\"my-class\"><div>c0</div><div>c1</div><div>c2</div></div>"
        );
    }

    #[test]
    fn link_shall_render() {
        let link = Link::from_text("https://example.com", "Example");
        assert_eq!(
            link.to_html(),
            "<a href=\"https://example.com\">Example</a>"
        );
    }

    #[test]
    fn link_with_h2_inside_shall_render() {
        let link = Link::from_child("https://example.com", Box::new(Text::h2("Example")));
        assert_eq!(
            link.to_html(),
            "<a href=\"https://example.com\"><h2>Example</h2></a>"
        );
    }

    #[test]
    fn div_with_text_with_brackets_shall_be_escaped() {
        let div = Div::new()
            .with_class("my-class")
            .with_text("<Hello, World!>");
        assert_eq!(
            div.to_html(),
            "<div class=\"my-class\">&lt;Hello, World!&gt;</div>"
        );
    }

    #[test]
    fn link_content_shall_be_escaped() {
        let link = Link::from_text("https://<example>.com", "<Example>");
        assert_eq!(
            link.to_html(),
            "<a href=\"https://<example>.com\">&lt;Example&gt;</a>"
        );
    }

    #[test]
    fn img_shall_render() {
        let img = Img::new("https://example.com/image.png", "Example");
        assert_eq!(
            img.to_html(),
            "<img src=\"https://example.com/image.png\" alt=\"Example\" />"
        );
    }

    #[test]
    fn img_alt_text_shall_be_escaped() {
        let img = Img::new("https://example.com/image.png", "<Example>");
        assert_eq!(
            img.to_html(),
            "<img src=\"https://example.com/image.png\" alt=\"&lt;Example&gt;\" />"
        );
    }

    #[test]
    fn div_with_img_shall_render() {
        let div = Div::new()
            .with_class("my-class")
            .with_child(Img::new("https://example.com/image.png", "Example"));
        assert_eq!(
            div.to_html(),
            "<div class=\"my-class\"><img src=\"https://example.com/image.png\" alt=\"Example\" /></div>"
        );
    }

    #[test]
    fn pre_shall_render() {
        let pre = Pre::new("Hello, World!");
        assert_eq!(pre.to_html(), "<pre>Hello, World!</pre>");
    }

    #[test]
    fn empty_table_shall_render() {
        let table = Table::new();
        assert_eq!(table.to_html(), "<table></table>");
    }

    #[test]
    fn table_with_2_rows_shall_render() {
        let table = Table::new()
            .with_row(
                Row::new()
                    .with_cell(Text::new("r1c1"))
                    .with_cell(Text::new("r1c2")),
            )
            .with_row(
                Row::new()
                    .with_cell(Text::new("r2c1"))
                    .with_cell(Text::new("r2c2")),
            );
        assert_eq!(
            table.to_html(),
            "<table><tr><td>r1c1</td><td>r1c2</td></tr><tr><td>r2c1</td><td>r2c2</td></tr></table>"
        );
    }

    #[test]
    fn table_built_from_row_iterator_shall_render() {
        let rows = vec![
            Row::new()
                .with_cell(Text::new("r1c1"))
                .with_cell(Text::new("r1c2")),
            Row::new()
                .with_cell(Text::new("r2c1"))
                .with_cell(Text::new("r2c2")),
        ];

        let table = Table::new().with_rows(rows.into_iter());
        assert_eq!(
            table.to_html(),
            "<table><tr><td>r1c1</td><td>r1c2</td></tr><tr><td>r2c1</td><td>r2c2</td></tr></table>"
        );
    }

    #[test]
    fn gauge_shall_render() {
        let gauge = Gauge::new(Some(50.145), "Example");
        assert_eq!(
            gauge.to_html(),
            r#"<div class="gauge"><div class="container"><div class="gauge-a"></div><div class="gauge-b"></div><div class="gauge-c bg-5" style="transform: rotate(0.25turn)"></div><div class="gauge-data"><span class="percent">50.15%</h1></div></div><div>Example</div></div>"#
        );
    }

    #[test]
    fn gauge_with_none_shall_render() {
        let gauge = Gauge::new(None, "Example");
        assert_eq!(
            gauge.to_html(),
            r#"<div class="gauge"><div class="container"><div class="gauge-a"></div><div class="gauge-b"></div><div class="gauge-c bg-none" style="transform: rotate(0.00turn)"></div><div class="gauge-data"><span class="percent">-</h1></div></div><div>Example</div></div>"#
        );
    }
}
