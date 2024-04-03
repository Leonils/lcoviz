use super::{
    components::ComponentsFactory, file_lines_provider::FileLinesProvider, html_builder::HtmlNode,
};

pub trait ToHtml {
    fn to_html(&self, components: impl ComponentsFactory) -> HtmlNode;
}

pub trait ToHtmlWithLinesProvider {
    fn to_html(
        &self,
        components: impl ComponentsFactory,
        lines_provider: impl FileLinesProvider,
    ) -> HtmlNode;
}
