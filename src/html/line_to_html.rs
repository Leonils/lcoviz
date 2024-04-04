use crate::models::{
    components::ComponentsFactory, file_lines_provider::FileLinesProvider, html_builder::HtmlNode,
    to_html::ToHtmlWithLinesProvider,
};
use lcov::report::section::line::{self, Lines};

impl ToHtmlWithLinesProvider for Lines {
    fn to_html(
        &self,
        components: impl ComponentsFactory,
        lines_provider: impl FileLinesProvider,
    ) -> HtmlNode {
        let lines = lines_provider
            .get_file_lines()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, line_content)| {
                let key = line::Key {
                    line: (i + 1) as u32,
                };

                let count = match self.get(&key) {
                    None => 0,
                    Some(line::Value { count, checksum: _ }) => *count,
                };

                components.create_line((i + 1) as u32, count, line_content.to_string())
            })
            .collect();

        components.create_code(lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::mocks::{MockComponentsFactory, MockFilesProvider};
    use lcov::report::section::line::{Key, Lines, Value};
    use std::collections::BTreeMap;

    #[test]
    fn test_no_lines_to_html() {
        let lines: Lines = BTreeMap::new();
        let html = lines.to_html(MockComponentsFactory {}, MockFilesProvider::new(0));
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

        let html = lines.to_html(MockComponentsFactory {}, MockFilesProvider::new(2));
        assert_eq!(
            html.render(),
            "<div>line(1, 1, line_1);line(2, 4, line_2);</div>"
        );
    }
}
