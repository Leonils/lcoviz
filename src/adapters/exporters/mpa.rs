use std::{error::Error, io::Write, path::PathBuf};

use crate::{
    aggregation::tested_root::TestedRoot,
    core::{Exporter, Renderer, TestedContainer, TestedFile, WithPath},
    file_provider::LocalFileLinesProvider,
};

pub struct MpaExporter<TRenderer: Renderer> {
    renderer: TRenderer,
    root: TestedRoot,
    output_path_root: PathBuf,
}
impl<'a, TRenderer: Renderer> MpaExporter<TRenderer> {
    pub fn new(renderer: TRenderer, root: TestedRoot, output_path_root: PathBuf) -> Self {
        MpaExporter {
            renderer,
            root,
            output_path_root,
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

        std::fs::create_dir_all(target_path.parent().unwrap())?;

        let mut f = std::fs::File::create(target_path)?;
        f.write_all(
            self.renderer
                .render_file_coverage_details(root, file, lines_provider)
                .as_bytes(),
        )?;

        Ok(())
    }

    fn render_module(&self, root: &impl WithPath, module: &impl TestedContainer) -> () {
        let relative_path_root_to_module = module.get_path_relative_to(&self.root.get_path());

        let output_path = self.output_path_root.join(relative_path_root_to_module);
        std::fs::create_dir_all(&output_path).unwrap();
        let mut file = std::fs::File::create(output_path.join("index.html"))
            .expect("Failed to create index.html");
        file.write_all(
            self.renderer
                .render_module_coverage_details(root, module)
                .as_bytes(),
        )
        .unwrap();

        for child in module.get_container_children() {
            self.render_module(root, child);
        }

        for file in module.get_code_file_children() {
            self.render_file(root, file).unwrap();
        }
    }
}

impl<TRenderer: Renderer> Exporter for MpaExporter<TRenderer> {
    fn render_root(self) -> () {
        self.render_module(&self.root, &self.root);
    }
}

#[cfg(test)]
mod test {}
