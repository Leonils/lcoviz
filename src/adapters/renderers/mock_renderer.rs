use crate::{
    core::{Renderer, TestedContainer, TestedFile, WithPath},
    file_provider::FileLinesProvider,
};

pub struct MockRenderer;
impl Renderer for MockRenderer {
    fn render_module_coverage_details(
        &self,
        _root: &impl WithPath,
        module: &impl TestedContainer,
    ) -> String {
        format!("Report for module {}", module.get_name())
    }

    fn render_file_coverage_details(
        &self,
        _root: &impl WithPath,
        file: &impl TestedFile,
        _file_provider: &impl FileLinesProvider,
    ) -> String {
        format!("Report for file {}", file.get_name())
    }
}
