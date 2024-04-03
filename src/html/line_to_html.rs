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
        let lines: Vec<HtmlNode> = self.keys()
            .map(|line| {
                let count = self.get(line).unwrap().count;
                let line_content = lines_provider
                    .get_file_lines(line.line as usize, line.line as usize + 1)
                    .unwrap_or_else(|_| "".to_string());
                components.create_line(line.line, count, line_content)
            })
            .collect();
        
        components.create_code(lines)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use lcov::report::section::line::{Key, Lines, Value};
    use crate::mocks::{MockComponentsFactory, MockFilesProvider};
    use super::*;

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
            "<div>line(1, 1, line_1);line(2, 4, line_2);</div>"
        );
    }
}
