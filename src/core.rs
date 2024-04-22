use std::{
    error::Error,
    io::Write,
    path::{Path, PathBuf},
};

#[cfg(test)]
use mockall::automock;

use pathdiff::diff_paths;

use crate::file_provider::FileLinesProvider;

#[derive(Default, Debug, PartialEq)]
pub struct AggregatedCoverageCounters {
    pub count: u32,
    pub covered_count: u32,
}
impl AggregatedCoverageCounters {
    pub fn new(count: u32, covered_count: u32) -> Self {
        AggregatedCoverageCounters {
            count,
            covered_count,
        }
    }

    pub fn percentage(&self) -> Option<f32> {
        if self.count == 0 {
            return None;
        }
        Some((self.covered_count as f32 / self.count as f32) * 100.0)
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct AggregatedCoverage {
    pub lines: AggregatedCoverageCounters,
    pub functions: AggregatedCoverageCounters,
    pub branches: AggregatedCoverageCounters,
}

pub trait TestedFile: WithPath {
    fn get_aggregated_coverage(&self) -> &AggregatedCoverage;
    fn get_line_coverage(&self, line: u32) -> Option<u64>;
}

pub trait TestedContainer: WithPath {
    fn get_aggregated_coverage(&self) -> &AggregatedCoverage;
    fn get_container_children(&self) -> impl Iterator<Item = &impl TestedContainer>;
    fn get_code_file_children(&self) -> impl Iterator<Item = &impl TestedFile>;
}

pub trait Renderer {
    fn render_module_coverage_details(
        &self,
        root: &impl WithPath,
        module: &impl TestedContainer,
        links_computer: &impl LinksComputer,
    ) -> String;
    fn render_file_coverage_details(
        &self,
        root: &impl WithPath,
        file: &impl TestedFile,
        file_provider: &impl FileLinesProvider,
        links_computer: &impl LinksComputer,
    ) -> String;
}

pub trait Exporter {
    fn render_root(self) -> ();
}

pub trait WithPath {
    fn get_name(&self) -> &str;
    fn get_path_string(&self) -> String;
    fn get_path(&self) -> PathBuf {
        PathBuf::from(self.get_path_string())
    }
    fn get_path_relative_to(&self, source: &PathBuf) -> PathBuf {
        diff_paths(self.get_path(), source).unwrap()
    }
}

pub struct LinkPayload {
    pub link: String,
    pub text: String,
}
pub trait LinksComputer {
    fn get_links_from_file(
        &self,
        root: &impl WithPath,
        file: &impl WithPath,
    ) -> impl Iterator<Item = LinkPayload>;
}

#[cfg_attr(test, automock)]
pub trait FileSystem {
    fn create_dir_all(&self, path: &Path) -> Result<(), Box<dyn Error>>;
    fn write_all(&self, path: &Path, content: &str) -> Result<(), Box<dyn Error>>;
    fn is_dir(&self, path: &Path) -> bool {
        path.is_dir()
    }
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
