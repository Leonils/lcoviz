use std::{error::Error, path::PathBuf};

use crate::core::{Exporter, FileSystem, Renderer, TestedContainer};

pub struct SpaExporter<'a, TRenderer: Renderer, TFileSystem: FileSystem, TRoot: TestedContainer> {
    renderer: TRenderer,
    root: TRoot,
    output_path_root: &'a PathBuf,
    file_system: &'a TFileSystem,
}
impl<'a, TRenderer: Renderer, TFileSystem: FileSystem, TRoot: TestedContainer>
    SpaExporter<'a, TRenderer, TFileSystem, TRoot>
{
    pub fn new(
        renderer: TRenderer,
        root: TRoot,
        output_path_root: &'a PathBuf,
        file_system: &'a TFileSystem,
    ) -> Self {
        SpaExporter {
            renderer,
            root,
            output_path_root,
            file_system,
        }
    }

    fn render(&self) -> Result<(), Box<dyn Error>> {
        let relative_path_root_to_module = self.root.get_path();

        let output_path = self.output_path_root.join(relative_path_root_to_module);
        self.file_system.create_dir_all(&output_path)?;

        self.file_system.write_all(
            &output_path.join("coverage.txt"),
            &self
                .renderer
                .render_module_coverage_details(&self.root, &self.root),
        )?;

        Ok(())
    }
}

impl<'a, TRenderer: Renderer, TFileSystem: FileSystem, TRoot: TestedContainer> Exporter
    for SpaExporter<'a, TRenderer, TFileSystem, TRoot>
{
    fn render_root(self) -> () {
        self.render().expect(&format!(
            "Failed to render root to {}:",
            self.output_path_root.display()
        ));
    }
}

#[cfg(test)]
mod test {

    use crate::{
        adapters::renderers::mock_renderer::MockRenderer, aggregation::tested_root::TestedRoot,
        core::MockFileSystem,
    };

    use super::*;
    use std::path::{Path, PathBuf};

    macro_rules! expect_create_dir_all {
        ($mock_filesystem:ident, $times:expr, $path:expr) => {
            $mock_filesystem
                .expect_create_dir_all()
                .times($times)
                .withf(move |path| path == Path::new($path))
                .returning(|_| Ok(()));
        };
    }

    macro_rules! expect_write_all {
        ($mock_filesystem:ident, $path:expr, $content:expr) => {
            $mock_filesystem
                .expect_write_all()
                .times(1)
                .withf(move |path, content| path == Path::new($path) && content == $content)
                .returning(|_, _| Ok(()));
        };
    }

    #[test]
    fn export_empty_report_shall_generate_a_single_coverage_text() {
        let empty_report = TestedRoot::default();
        let output_path = PathBuf::from("target");

        let mut fs = MockFileSystem::new();
        expect_create_dir_all!(fs, 1, "target");
        expect_write_all!(fs, "target/coverage.txt", "Report for module ");

        let exporter = SpaExporter::new(MockRenderer, empty_report, &output_path, &fs);
        exporter.render_root();
    }
}
