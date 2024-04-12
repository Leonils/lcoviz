use crate::core::{AggregatedCoverage, AggregatedCoverageCounters};

use super::{test_report::ReportTree, tested_file::TestedFile};

pub struct AggregatedFixtures {}
impl AggregatedFixtures {
    pub fn get_empty_report() -> ReportTree {
        ReportTree::default()
    }

    pub fn get_top_level_file_report_no_line() -> ReportTree {
        let main_cpp = TestedFile::new("main.cpp", "main.cpp");
        let report = ReportTree::from_source_files(vec![main_cpp]);
        report
    }

    /// Build an aggregate with 10 lines, 5 covered, 2 functions, 1 covered, 3 branches, 2 covered
    pub fn get_file_aggregates_10_5() -> AggregatedCoverage {
        AggregatedCoverage {
            lines: AggregatedCoverageCounters::new(10, 5),
            functions: AggregatedCoverageCounters::new(2, 1),
            branches: AggregatedCoverageCounters::new(3, 2),
        }
    }

    /// Build an aggregate with 20 lines, 10 covered, 7 functions, 6 covered, 0 branches, 0 covered
    pub fn get_file_aggregates_20_10() -> AggregatedCoverage {
        AggregatedCoverage {
            lines: AggregatedCoverageCounters::new(20, 10),
            functions: AggregatedCoverageCounters::new(7, 6),
            ..Default::default()
        }
    }

    /// Build an aggregate with 3 lines, 1 covered
    pub fn get_file_aggregates_3_1() -> AggregatedCoverage {
        AggregatedCoverage {
            lines: AggregatedCoverageCounters::new(3, 1),
            ..Default::default()
        }
    }

    pub fn get_top_level_file_report_with_aggregated() -> ReportTree {
        let main_cpp =
            TestedFile::with_aggregated("main.cpp", "main.cpp", Self::get_file_aggregates_10_5());
        let report = ReportTree::from_source_files(vec![main_cpp]);
        report
    }
}
