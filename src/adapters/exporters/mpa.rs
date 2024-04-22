#[cfg(test)]
use mockall::automock;

use std::{
    error::Error,
    io::Write,
    path::{Path, PathBuf},
};

use crate::{
    aggregation::tested_root::TestedRoot,
    core::{Exporter, Renderer, TestedContainer, TestedFile, WithPath},
    file_provider::LocalFileLinesProvider,
};

#[cfg_attr(test, automock)]
pub trait FileSystem {
    fn create_dir_all(&self, path: &Path) -> Result<(), Box<dyn Error>>;
    fn write_all(&self, path: &Path, content: &str) -> Result<(), Box<dyn Error>>;
}

pub struct LocalFileSystem;
impl FileSystem for LocalFileSystem {
    fn create_dir_all(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        std::fs::create_dir_all(path)?;
        Ok(())
    }

    fn write_all(&self, path: &Path, content: &str) -> Result<(), Box<dyn Error>> {
        let mut f = std::fs::File::create(path)?;
        f.write_all(content.as_bytes())?;
        Ok(())
    }
}

pub struct MpaExporter<TRenderer: Renderer, TFileSystem: FileSystem> {
    renderer: TRenderer,
    root: TestedRoot,
    output_path_root: PathBuf,
    file_system: TFileSystem,
}
impl<'a, TRenderer: Renderer, TFileSystem: FileSystem> MpaExporter<TRenderer, TFileSystem> {
    pub fn new(
        renderer: TRenderer,
        root: TestedRoot,
        output_path_root: PathBuf,
        file_system: TFileSystem,
    ) -> Self {
        MpaExporter {
            renderer,
            root,
            output_path_root,
            file_system,
        }
    }

    fn render_file(
        &self,
        root: &impl WithPath,
        file: &impl TestedFile,
    ) -> Result<(), Box<dyn Error>> {
        let lines_provider = LocalFileLinesProvider::new(file.get_path());
        let mut target_path = self
            .output_path_root
            .join(file.get_path_relative_to(&self.root.get_path()));

        let extension = target_path.extension().unwrap_or_default();
        target_path.set_extension(format!("{}.html", extension.to_string_lossy()));

        self.file_system
            .create_dir_all(target_path.parent().unwrap())?;

        self.file_system.write_all(
            &target_path,
            &self
                .renderer
                .render_file_coverage_details(root, file, &lines_provider),
        )?;

        Ok(())
    }

    fn render_module(
        &self,
        root: &impl WithPath,
        module: &impl TestedContainer,
    ) -> Result<(), Box<dyn Error>> {
        let relative_path_root_to_module = module.get_path_relative_to(&self.root.get_path());

        let output_path = self.output_path_root.join(relative_path_root_to_module);
        self.file_system.create_dir_all(&output_path)?;

        self.file_system.write_all(
            &output_path.join("index.html"),
            &self.renderer.render_module_coverage_details(root, module),
        )?;

        for child in module.get_container_children() {
            self.render_module(root, child)?;
        }

        for file in module.get_code_file_children() {
            self.render_file(root, file)?;
        }

        Ok(())
    }
}

impl<TRenderer: Renderer, TFileSystem: FileSystem> Exporter
    for MpaExporter<TRenderer, TFileSystem>
{
    fn render_root(self) -> () {
        self.render_module(&self.root, &self.root).unwrap();
    }
}

#[cfg(test)]
mod test {

    use crate::{
        adapters::renderers::mock_renderer::MockRenderer, aggregation::fixtures::AggregatedFixtures,
    };

    use super::*;
    use std::path::PathBuf;

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
    fn export_empty_report_shall_generate_a_single_index_html() {
        let empty_report = TestedRoot::default();
        let output_path = PathBuf::from("target");

        let mut fs = MockFileSystem::new();
        expect_create_dir_all!(fs, 1, "target");
        expect_write_all!(fs, "target/index.html", "Report for module Test report");

        let exporter = MpaExporter::new(MockRenderer, empty_report, output_path, fs);
        exporter.render_root();
    }

    #[test]
    fn export_report_with_top_level_file() {
        let report = AggregatedFixtures::get_top_level_file_report_no_line();
        let output_path = PathBuf::from("target");

        let mut fs = MockFileSystem::new();
        expect_create_dir_all!(fs, 2, "target");
        expect_write_all!(fs, "target/index.html", "Report for module Test report");
        expect_write_all!(fs, "target/main.cpp.html", "Report for file main.cpp");

        let exporter = MpaExporter::new(MockRenderer, report, output_path, fs);
        exporter.render_root();
    }

    #[test]
    fn export_report_with_top_level_file_and_module() {
        let report = AggregatedFixtures::get_nested_file_in_report();
        let output_path = PathBuf::from("target");

        let mut fs = MockFileSystem::new();
        expect_create_dir_all!(fs, 2, "target");
        expect_create_dir_all!(fs, 2, "target/module");
        expect_write_all!(fs, "target/index.html", "Report for module Test report");
        expect_write_all!(fs, "target/module/index.html", "Report for module module");
        expect_write_all!(fs, "target/main.cpp.html", "Report for file main.cpp");
        expect_write_all!(
            fs,
            "target/module/nested.cpp.html",
            "Report for file nested.cpp"
        );

        let exporter = MpaExporter::new(MockRenderer, report, output_path, fs);
        exporter.render_root();
    }
}
