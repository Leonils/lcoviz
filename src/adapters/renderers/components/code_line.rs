use crate::{
    core::TestedFile,
    html::components::{Pre, Row, Table, Text, ToHtml},
};

pub struct CodeLines<'a, TFile: TestedFile> {
    file: &'a TFile,
    lines: Vec<String>,
}
impl<'a, TFile: TestedFile> CodeLines<'a, TFile> {
    pub fn new(file: &'a TFile, lines: Vec<String>) -> Self {
        Self { file, lines }
    }

    fn render_line(&self, line_number: usize) -> Row {
        let coverage = self.file.get_line_coverage(line_number as u32);
        let empty_line = String::new();
        let line = self.lines.get(line_number).unwrap_or(&empty_line);

        let class = match coverage {
            Some(cov) if cov > 0 => "line-covered",
            Some(_) => "line-not-covered",
            None => "line-not-tested",
        };

        Row::new()
            .with_class(class)
            .with_cell(Text::new(&(line_number + 1).to_string()))
            .with_cell(Text::new(
                &coverage.map(|c| c.to_string()).unwrap_or_default(),
            ))
            .with_cell(Pre::new(&line))
    }

    fn render_lines(&self) -> Table {
        let rows = self
            .lines
            .iter()
            .enumerate()
            .map(|(i, _)| self.render_line(i));

        Table::new().with_rows(rows)
    }
}
impl<'a, TFile: TestedFile> ToHtml for CodeLines<'a, TFile> {
    fn to_html(&self) -> String {
        self.render_lines().to_html()
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::*;
    use crate::{
        aggregation::tested_file::TestedCodeFile, assert_html_eq, test_utils::builders::InsertLine,
    };
    use lcov::report::section::{Key as SectionKey, Value as SectionValue};

    #[test]
    fn test_render_lines() {
        let section = SectionValue::default()
            .insert_line(1, 3)
            .insert_line(2, 1)
            .insert_line(3, 0)
            .insert_line(4, 2);
        let key = SectionKey {
            source_file: PathBuf::from("file.cpp"),
            test_name: String::from(""),
        };
        let file = TestedCodeFile::from_section(key, section, "", "");
        let lines = CodeLines::new(
            &file,
            vec![
                String::from("line 1"),
                String::from("line 2"),
                String::from("line 3"),
                String::from("line 4"),
            ],
        );

        assert_html_eq!(
            lines.to_html(),
            "<table>",
            r#"<tr class="line-not-tested"><td>1</td><td></td><td><pre>line 1</pre></td></tr>"#,
            r#"<tr class="line-covered"><td>2</td><td>3</td><td><pre>line 2</pre></td></tr>"#,
            r#"<tr class="line-covered"><td>3</td><td>1</td><td><pre>line 3</pre></td></tr>"#,
            r#"<tr class="line-not-covered"><td>4</td><td>0</td><td><pre>line 4</pre></td></tr>"#,
            "</table>"
        );
    }
}
