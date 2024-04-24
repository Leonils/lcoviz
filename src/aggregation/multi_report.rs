use crate::core::{AggregatedCoverage, TestedContainer, TestedFile, WithPath};

use super::{tested_file::TestedCodeFile, tested_root::TestedRoot};

pub struct MultiReport {
    reports: Vec<TestedRoot>,
    aggregated: AggregatedCoverage,
}

impl MultiReport {
    pub fn new() -> Self {
        MultiReport {
            reports: Vec::new(),
            aggregated: AggregatedCoverage::default(),
        }
    }

    pub fn add_report(&mut self, report: TestedRoot) {
        self.aggregated.add(&report.get_aggregated_coverage());
        self.reports.push(report);
    }
}

impl WithPath for MultiReport {
    fn get_name(&self) -> &str {
        "MultiReport"
    }

    fn get_path(&self) -> std::path::PathBuf {
        std::path::PathBuf::from("")
    }

    fn get_path_string(&self) -> String {
        "".to_string()
    }

    fn is_dir(&self) -> bool {
        true
    }
}

impl TestedContainer for MultiReport {
    fn get_aggregated_coverage(&self) -> &AggregatedCoverage {
        &self.aggregated
    }

    fn get_code_file_children(&self) -> impl Iterator<Item = &impl TestedFile> {
        [].iter() as std::slice::Iter<'_, TestedCodeFile>
    }

    fn get_container_children(&self) -> impl Iterator<Item = &impl TestedContainer> {
        self.reports.iter()
    }
}
