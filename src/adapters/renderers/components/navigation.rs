use crate::{
    core::{LinkPayload, LinksComputer, WithPath},
    html::components::{Div, Link, ToHtml},
};

pub struct Navigation<'a, TLinksComputer: LinksComputer, TRoot: WithPath, TFile: WithPath> {
    links_computer: &'a TLinksComputer,
    root: &'a TRoot,
    file: &'a TFile,
}

impl<'a, TLinksComputer: LinksComputer, TRoot: WithPath, TFile: WithPath>
    Navigation<'a, TLinksComputer, TRoot, TFile>
{
    pub fn new(
        links_computer: &'a TLinksComputer,
        root: &'a TRoot,
        file: &'a TFile,
    ) -> Navigation<'a, TLinksComputer, TRoot, TFile> {
        Navigation {
            links_computer,
            root,
            file,
        }
    }

    fn render_navigation(&self) -> Div {
        let links = self
            .links_computer
            .get_links_from_file(self.root, self.file)
            .collect::<Vec<LinkPayload>>();

        if links.is_empty() {
            return Div::new();
        }

        let links = links
            .into_iter()
            .map(|link| {
                vec![
                    Div::new()
                        .with_class("navigation-part")
                        .with_child(Link::from_link_payload(link)),
                    Div::new().with_text(" / "),
                ]
            })
            .flatten();

        Div::new()
            .with_class("navigation")
            .with_children(links)
            .with_child(
                Div::new()
                    .with_class("navigation-part")
                    .with_text(self.file.get_name()),
            )
    }
}

impl<'a, TLinksComputer: LinksComputer, TRoot: WithPath, TFile: WithPath> ToHtml
    for Navigation<'a, TLinksComputer, TRoot, TFile>
{
    fn to_html(&self) -> String {
        self.render_navigation().to_html()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        adapters::exporters::mpa_links::MpaLinksComputer, aggregation::tested_file::TestedCodeFile,
        assert_html_eq,
    };

    use super::*;

    #[test]
    fn test_navigation_from_root() {
        let links_computer = MpaLinksComputer;
        let root = TestedCodeFile::new("", "root");
        let navigation = Navigation::new(&links_computer, &root, &root);

        assert_html_eq!(navigation.to_html(), r#"<div></div>"#);
    }

    #[test]
    fn test_navigation_from_top_level_file() {
        let links_computer = MpaLinksComputer;
        let root = TestedCodeFile::new("", "root");
        let file = TestedCodeFile::new("file", "file");
        let navigation = Navigation::new(&links_computer, &root, &file);

        assert_html_eq!(
            navigation.to_html(),
            r#"<div class="navigation">"#,
            r#"<div class="navigation-part"><a href="index.html">root</a></div>"#,
            r#"<div> / </div>"#,
            r#"<div class="navigation-part">file</div>"#,
            r#"</div>"#
        );
    }

    #[test]
    fn test_navigation_from_deeply_nested_file() {
        let links_computer = MpaLinksComputer;
        let root = TestedCodeFile::new("", "root");
        let file = TestedCodeFile::new("some/lib/to/file", "file");
        let navigation = Navigation::new(&links_computer, &root, &file);

        assert_html_eq!(
            navigation.to_html(),
            r#"<div class="navigation">"#,
            r#"<div class="navigation-part"><a href="../../../index.html">root</a></div>"#,
            r#"<div> / </div>"#,
            r#"<div class="navigation-part"><a href="../../index.html">some</a></div>"#,
            r#"<div> / </div>"#,
            r#"<div class="navigation-part"><a href="../index.html">lib</a></div>"#,
            r#"<div> / </div>"#,
            r#"<div class="navigation-part"><a href="index.html">to</a></div>"#,
            r#"<div> / </div>"#,
            r#"<div class="navigation-part">file</div>"#,
            r#"</div>"#
        );
    }
}
