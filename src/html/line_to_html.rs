use lcov::report::section::line::Lines;

use crate::models::{
    components::ComponentsFactory, file_lines_provider::FileLinesProvider, html_builder::HtmlNode,
    to_html::ToHtmlWithLinesProvider,
};

impl ToHtmlWithLinesProvider for Lines {
    fn to_html(
        &self,
        components: impl ComponentsFactory,
        lines_provider: impl FileLinesProvider,
    ) -> HtmlNode {
        let mut container = HtmlNode::div();

        for line in self.keys() {
            let count = self.get(line).unwrap().count;
            let line_content = lines_provider
                .get_file_lines(line.line as usize, line.line as usize + 1)
                .unwrap_or_else(|_| "".to_string());
            container.add_child(components.create_line(line.line, count, line_content));
        }

        container
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use lcov::report::section::line::{Key, Lines, Value};

    use crate::models::html_builder::HtmlNode;

    use super::*;

    // Mock file provider
    struct MockFilesProvider;
    impl FileLinesProvider for MockFilesProvider {
        fn get_file_lines(
            &self,
            start_line: usize,
            end_line: usize,
        ) -> Result<String, std::io::Error> {
            let mut lines: Vec<String> = Vec::with_capacity(end_line - start_line + 1);
            for i in start_line..end_line {
                lines.push(format!("line_{}", i));
            }
            Ok(lines.join("\n"))
        }
    }

    // Mock components factory
    struct MockComponentsFactory;
    impl ComponentsFactory for MockComponentsFactory {
        fn create_line(
            &self,
            line_number: u32,
            count_number: u64,
            line_content: String,
        ) -> HtmlNode {
            HtmlNode::text(
                format!(
                    "<>Line {}[{}]: {}</>",
                    line_number, count_number, line_content
                )
                .as_str(),
            )
        }
    }

    #[test]
    fn test_no_lines_to_html() {
        let lines: Lines = BTreeMap::new();
        let html = lines.to_html(MockComponentsFactory {}, MockFilesProvider {});
        assert_eq!(html.render(), "<div></div>");
    }

    #[test]
    fn test_lines_to_html() {
        let line1_key = Key { line: 1 };
        let line2_key = Key { line: 2 };
        let line1_value = Value {
            count: 1,
            checksum: None,
        };
        let line2_value = Value {
            count: 4,
            checksum: None,
        };

        let mut lines: Lines = Lines::new();
        lines.insert(line1_key, line1_value);
        lines.insert(line2_key, line2_value);

        let html = lines.to_html(MockComponentsFactory {}, MockFilesProvider {});
        assert_eq!(
            html.render(),
            "<div><>Line 1[1]: line_1</><>Line 2[4]: line_2</></div>"
        );
    }
}
