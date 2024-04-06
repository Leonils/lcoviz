use super::{aggregated::Aggregated, test_report::ReportTree, tested_file::TestedFile};

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
    pub fn get_file_aggregates_10_5() -> Aggregated {
        Aggregated {
            lines_count: 10,
            covered_lines_count: 5,
            functions_count: 2,
            covered_functions_count: 1,
            branches_count: 3,
            covered_branches_count: 2,
        }
    }

    /// Build an aggregate with 20 lines, 10 covered, 7 functions, 6 covered, 0 branches, 0 covered
    pub fn get_file_aggregates_20_10() -> Aggregated {
        Aggregated {
            lines_count: 20,
            covered_lines_count: 10,
            functions_count: 7,
            covered_functions_count: 6,
            branches_count: 0,
            covered_branches_count: 0,
        }
    }

    pub fn get_top_level_file_report_with_aggregated() -> ReportTree {
        let main_cpp =
            TestedFile::with_aggregated("main.cpp", "main.cpp", Self::get_file_aggregates_10_5());
        let report = ReportTree::from_source_files(vec![main_cpp]);
        report
    }
}
