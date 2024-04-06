use crate::{
    aggregation::test_report::ReportTree,
    models::{components::ComponentsFactory, html_builder::HtmlNode, to_html::ToHtml},
};

impl ToHtml for ReportTree {
    fn to_html(&self, components: impl ComponentsFactory) -> HtmlNode {
        let mut body = HtmlNode::new("body");
        body.add_child(components.create_header("Empty report"));
        body
    }
}

#[cfg(test)]
mod tests {
    use crate::{aggregation::test_report::ReportTree, styles::light::MockComponentsFactory};

    use super::*;

    #[test]
    fn generating_html_for_empty_report_shall_return_a_message() {
        let report = ReportTree::default();
        let component_factory = MockComponentsFactory {};
        let html = report.to_html(component_factory);
        assert_eq!(html.render(), "<body><h1>Empty report</h1></body>")
    }
}
