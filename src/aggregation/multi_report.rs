use crate::core::{AggregatedCoverage, TestedContainer, TestedFile};

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
