use lcov::report::section::line::Lines;

use crate::models::to_html::ToHtml;

impl ToHtml for Lines {
    fn to_html(&self) -> String {
        let mut html = String::new();

        html.push_str("<div className=\"lines\">\n");
        for line in self.keys() {
            let count = self.get(line).unwrap().count;
            html.push_str(format!("    <div>Line {}: {}</div>\n", line.line, count).as_str());
        }
        html.push_str("</div>\n");

        html
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use lcov::report::section::line::{Key, Lines, Value};

    use super::*;

    #[test]
    fn test_no_lines_to_html() {
        let lines: Lines = BTreeMap::new();
        let html = lines.to_html();
        assert_eq!(html, "<div className=\"lines\">\n</div>\n");
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

        let html = lines.to_html();
        assert_eq!(
            html,
            "<div className=\"lines\">
    <div>Line 1: 1</div>
    <div>Line 2: 4</div>
</div>
"
        );
    }
}
