use std::{error::Error, path::PathBuf};

use crate::{
    aggregation::tested_root::TestedRoot,
    core::{Exporter, FileSystem, Renderer, TestedContainer, TestedFile, WithPath},
    file_provider::LocalFileLinesProvider,
};

pub struct MpaExporter<'a, TRenderer: Renderer, TFileSystem: FileSystem> {
    renderer: TRenderer,
    root: TestedRoot,
    output_path_root: PathBuf,
    file_system: &'a TFileSystem,
}
impl<'a, TRenderer: Renderer, TFileSystem: FileSystem> MpaExporter<'a, TRenderer, TFileSystem> {
    pub fn new(
        renderer: TRenderer,
        root: TestedRoot,
        output_path_root: PathBuf,
        file_system: &'a TFileSystem,
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

impl<'a, TRenderer: Renderer, TFileSystem: FileSystem> Exporter
    for MpaExporter<'a, TRenderer, TFileSystem>
{
    fn render_root(self) -> () {
        self.render_module(&self.root, &self.root).unwrap();

        let required_resources = self.renderer.get_required_resources(&self.root);
        self.file_system
            .create_dir_all(&self.output_path_root.join("_resources"))
            .unwrap();
        for (resource_name, resource_content) in required_resources {
            let target_path = self.output_path_root.join("_resources").join(resource_name);
            self.file_system
                .write_all(&target_path, resource_content)
                .unwrap();
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{
        adapters::renderers::mock_renderer::MockRenderer,
        aggregation::{fixtures::AggregatedFixtures, tested_root::TestedRoot},
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
    fn export_empty_report_shall_generate_a_single_index_html() {
        let empty_report = TestedRoot::default();
        let output_path = PathBuf::from("target");

        let mut fs = MockFileSystem::new();
        expect_create_dir_all!(fs, 1, "target");
        expect_create_dir_all!(fs, 1, "target/_resources");
        expect_write_all!(fs, "target/index.html", "Report for module ");
        expect_write_all!(fs, "target/_resources/resource.svg", "<svg>...</svg>");

        let exporter = MpaExporter::new(MockRenderer, empty_report, output_path, &fs);
        exporter.render_root();
    }

    #[test]
    fn export_report_with_top_level_file() {
        let report = AggregatedFixtures::get_top_level_file_report_no_line();
        let output_path = PathBuf::from("target");

        let mut fs = MockFileSystem::new();
        expect_create_dir_all!(fs, 2, "target");
        expect_create_dir_all!(fs, 1, "target/_resources");
        expect_write_all!(fs, "target/index.html", "Report for module Test report");
        expect_write_all!(fs, "target/main.cpp.html", "Report for file main.cpp");
        expect_write_all!(fs, "target/_resources/resource.svg", "<svg>...</svg>");

        let exporter = MpaExporter::new(MockRenderer, report, output_path, &fs);
        exporter.render_root();
    }

    #[test]
    fn export_report_with_top_level_file_and_module() {
        let report = AggregatedFixtures::get_nested_file_in_report();
        let output_path = PathBuf::from("target");

        let mut fs = MockFileSystem::new();
        expect_create_dir_all!(fs, 2, "target");
        expect_create_dir_all!(fs, 2, "target/module");
        expect_create_dir_all!(fs, 1, "target/_resources");
        expect_write_all!(fs, "target/index.html", "Report for module Test report");
        expect_write_all!(fs, "target/module/index.html", "Report for module module");
        expect_write_all!(fs, "target/main.cpp.html", "Report for file main.cpp");
        expect_write_all!(
            fs,
            "target/module/nested.cpp.html",
            "Report for file nested.cpp"
        );
        expect_write_all!(fs, "target/_resources/resource.svg", "<svg>...</svg>");

        let exporter = MpaExporter::new(MockRenderer, report, output_path, &fs);
        exporter.render_root();
    }
}
