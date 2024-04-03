use lcov::Report;

use crate::{file_provider::LocalFileLinesProvider, models::to_html::ToHtml};

impl ToHtml for Report {
    fn to_html(&self) -> String {
        let mut html = String::new();

        for (section_key, section) in &self.sections {
            let local_file_line_provider =
                LocalFileLinesProvider::new(section_key.source_file.clone());
            html.push_str(format!("<div>Section: {:?}</div>\n", section_key.source_file).as_str());
        }

        html
    }
}

#[cfg(test)]
mod tests {
    use lcov::report::section;

    use super::*;

    #[test]
    fn test_report_to_html() {
        let mut report = Report::new();
        report.sections.insert(
            section::Key {
                test_name: "test".to_string(),
                source_file: "tests/fixtures/my_code.cpp".into(),
            },
            section::Value::default(),
        );

        let html = report.to_html();
        assert_eq!(html, "<div>Section: \"tests/fixtures/my_code.cpp\"</div>\n");
    }
}
