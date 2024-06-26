use super::{tested_file::TestedCodeFile, tested_module::TestedModule, tested_root::TestedRoot};
use crate::{
    core::{AggregatedCoverage, AggregatedCoverageCounters},
    test_utils::builders::{InsertBranch, InsertFunction, InsertLine, InsertSection},
};
use lcov::report::section::Value as SectionValue;

pub struct AggregatedFixtures {}
impl AggregatedFixtures {
    pub fn get_top_level_file_report_no_line() -> TestedRoot {
        let main_cpp = TestedCodeFile::new("main.cpp", "main.cpp");
        let report = TestedRoot::from_source_files(vec![main_cpp]);
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

    pub fn get_nested_file_in_report() -> TestedRoot {
        let main_cpp = TestedCodeFile::new("main.cpp", "main.cpp");
        let nested_cpp = TestedCodeFile::new("module/nested.cpp", "nested.cpp");
        let module = TestedModule::from_source_files("module", "module", vec![nested_cpp]);
        let root = TestedRoot::from_source_files_and_modules(vec![main_cpp], vec![module]);
        root
    }

    pub fn get_simple_full_report() -> TestedRoot {
        let section_main = SectionValue::default()
            .insert_line(1, 1)
            .insert_line(2, 0)
            .insert_line(3, 1)
            .insert_line(4, 2)
            .insert_branch(3, 0)
            .insert_branch(4, 1)
            .insert_function("f1", 1)
            .insert_function("f2", 2);

        let section_nested = SectionValue::default()
            .insert_line(1, 1)
            .insert_line(2, 2)
            .insert_function("f3", 1);

        let report = lcov::Report::new()
            .insert_section("main.cpp", section_main)
            .insert_section("module/nested.cpp", section_nested);

        let root = TestedRoot::from_original_report(report);
        root
    }
}
