use crate::{
    core::TestedFile,
    html::components::{Div, Img, Text, ToHtml},
};

pub struct FunctionDefs<'a, TFile: TestedFile> {
    file: &'a TFile,
    covered_svg: String,
    uncovered_svg: String,
}
impl<'a, TFile: TestedFile> FunctionDefs<'a, TFile> {
    pub fn new(file: &'a TFile, covered_svg: String, uncovered_svg: String) -> Self {
        Self {
            file,
            covered_svg,
            uncovered_svg,
        }
    }

    fn get_img_src(&self, count: u64) -> &str {
        if count > 0 {
            &self.covered_svg
        } else {
            &self.uncovered_svg
        }
    }

    fn get_class(count: u64) -> &'static str {
        if count > 0 {
            "function-covered"
        } else {
            "function-uncovered"
        }
    }

    fn get_img(&self, count: u64) -> Img {
        Img::new(self.get_img_src(count), "Function coverage")
    }

    fn render_function(&self, name: &str, count: u64) -> Div {
        Div::new()
            .with_class("function")
            .with_class(Self::get_class(count))
            .with_child(self.get_img(count))
            .with_child(
                Div::new()
                    .with_class("function-name")
                    .with_child(Text::new(name)),
            )
            .with_child(Div::new().with_class("fill"))
            .with_child(
                Div::new()
                    .with_class("function-hit")
                    .with_child(Text::new(&format!("{} calls", count))),
            )
    }

    fn render_functions(&self) -> Div {
        Div::new().with_class("functions").with_children(
            self.file
                .get_functions()
                .map(|(name, count)| self.render_function(&name, count)),
        )
    }
}

impl<'a, TFile: TestedFile> ToHtml for FunctionDefs<'a, TFile> {
    fn to_html(&self) -> String {
        self.render_functions().to_html()
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::*;
    use crate::{
        aggregation::tested_file::TestedCodeFile, assert_html_eq,
        test_utils::builders::InsertFunction,
    };
    use lcov::report::section::{Key as SectionKey, Value as SectionValue};
    #[test]
    fn render_one_uncovered_function() {
        let section = SectionValue::default().insert_function("f1", 0);
        let key = SectionKey {
            source_file: PathBuf::from("file.cpp"),
            test_name: String::from(""),
        };
        let file = TestedCodeFile::from_section(key, section, "", "");

        let covered_svg = "covered.svg".to_string();
        let uncovered_svg = "uncovered.svg".to_string();
        let functions = FunctionDefs::new(&file, covered_svg.clone(), uncovered_svg.clone());

        assert_html_eq!(
            functions.to_html(),
            r#"<div class="functions">"#,
            r#"<div class="function function-uncovered"><img src="uncovered.svg" alt="Function coverage" /><div class="function-name">f1</div><div class="fill"></div><div class="function-hit">0 calls</div></div>"#,
            r#"</div>"#
        );
    }

    #[test]
    fn render_one_covered_function() {
        let section = SectionValue::default().insert_function("f1", 1);
        let key = SectionKey {
            source_file: PathBuf::from("file.cpp"),
            test_name: String::from(""),
        };
        let file = TestedCodeFile::from_section(key, section, "", "");

        let covered_svg = "covered.svg".to_string();
        let uncovered_svg = "uncovered.svg".to_string();
        let functions = FunctionDefs::new(&file, covered_svg.clone(), uncovered_svg.clone());

        assert_html_eq!(
            functions.to_html(),
            r#"<div class="functions">"#,
            r#"<div class="function function-covered"><img src="covered.svg" alt="Function coverage" /><div class="function-name">f1</div><div class="fill"></div><div class="function-hit">1 calls</div></div>"#,
            r#"</div>"#
        );
    }

    #[test]
    fn test_render_several_functions() {
        let section = SectionValue::default()
            .insert_function("f1", 3)
            .insert_function("f2", 1)
            .insert_function("f3", 0)
            .insert_function("f4", 2);
        let key = SectionKey {
            source_file: PathBuf::from("file.cpp"),
            test_name: String::from(""),
        };
        let file = TestedCodeFile::from_section(key, section, "", "");

        let covered_svg = "covered.svg".to_string();
        let uncovered_svg = "uncovered.svg".to_string();
        let functions = FunctionDefs::new(&file, covered_svg.clone(), uncovered_svg.clone());

        assert_html_eq!(
            functions.to_html(),
            r#"<div class="functions">"#,
            r#"<div class="function function-covered"><img src="covered.svg" alt="Function coverage" /><div class="function-name">f1</div><div class="fill"></div><div class="function-hit">3 calls</div></div>"#,
            r#"<div class="function function-covered"><img src="covered.svg" alt="Function coverage" /><div class="function-name">f2</div><div class="fill"></div><div class="function-hit">1 calls</div></div>"#,
            r#"<div class="function function-uncovered"><img src="uncovered.svg" alt="Function coverage" /><div class="function-name">f3</div><div class="fill"></div><div class="function-hit">0 calls</div></div>"#,
            r#"<div class="function function-covered"><img src="covered.svg" alt="Function coverage" /><div class="function-name">f4</div><div class="fill"></div><div class="function-hit">2 calls</div></div>"#,
            r#"</div>"#
        );
    }
}
