use crate::core::{AggregatedCoverage, AggregatedCoverageCounters};

impl AggregatedCoverageCounters {
    fn add(&mut self, other: &Self) {
        self.count += other.count;
        self.covered_count += other.covered_count;
    }
}

impl AggregatedCoverage {
    pub fn add(&mut self, other: &Self) {
        self.lines.add(&other.lines);
        self.functions.add(&other.functions);
        self.branches.add(&other.branches);
    }

    pub fn from_section(value: &lcov::report::section::Value) -> Self {
        let lines_count = value.lines.len() as u32;
        let covered_lines_count = value
            .lines
            .iter()
            .filter(|(_, value)| value.count > 0)
            .count() as u32;

        let functions_count = value.functions.len() as u32;
        let covered_functions_count = value
            .functions
            .iter()
            .filter(|(_, value)| value.count > 0)
            .count() as u32;

        let branches_count = value.branches.len() as u32;
        let covered_branches_count = value
            .branches
            .iter()
            .filter(|(_, value)| value.taken.is_some_and(|taken| taken > 0))
            .count() as u32;

        Self {
            lines: AggregatedCoverageCounters::new(lines_count, covered_lines_count),
            functions: AggregatedCoverageCounters::new(functions_count, covered_functions_count),
            branches: AggregatedCoverageCounters::new(branches_count, covered_branches_count),
        }
    }
}

#[cfg(test)]
pub fn assert_aggregated_counters_eq(
    counters: &AggregatedCoverageCounters,
    count: u32,
    covered_count: u32,
) {
    assert_eq!(counters.count, count);
    assert_eq!(counters.covered_count, covered_count);
}

#[cfg(test)]
mod test {
    use lcov::report::section::Value as SectionValue;

    use crate::{
        aggregation::fixtures::AggregatedFixtures,
        core::AggregatedCoverage,
        test_utils::builders::{InsertBranch, InsertFunction, InsertLine},
    };

    use super::assert_aggregated_counters_eq;

    #[test]
    fn when_creating_an_aggregate_from_scratch_line_count_shall_be_0() {
        let aggregated = AggregatedCoverage::default();
        assert_aggregated_counters_eq(&aggregated.lines, 0, 0);
    }

    #[test]
    fn when_creating_an_aggregate_from_scratch_function_count_shall_be_0() {
        let aggregated = AggregatedCoverage::default();
        assert_aggregated_counters_eq(&aggregated.functions, 0, 0);
    }

    #[test]
    fn when_creating_an_aggregate_from_scratch_branch_count_shall_be_0() {
        let aggregated = AggregatedCoverage::default();
        assert_aggregated_counters_eq(&aggregated.branches, 0, 0);
    }

    #[test]
    fn when_adding_an_aggregate_to_another_the_counters_result_should_be_the_sum_of_both() {
        let mut aggregated = AggregatedFixtures::get_file_aggregates_10_5();
        let other = AggregatedFixtures::get_file_aggregates_20_10();
        aggregated.add(&other);

        assert_aggregated_counters_eq(&aggregated.lines, 30, 15);
        assert_aggregated_counters_eq(&aggregated.functions, 9, 7);
        assert_aggregated_counters_eq(&aggregated.branches, 3, 2);

        assert_aggregated_counters_eq(&other.lines, 20, 10);
        assert_aggregated_counters_eq(&other.functions, 7, 6);
        assert_aggregated_counters_eq(&other.branches, 0, 0);
    }

    #[test]
    fn when_creating_from_an_empty_section_line_counts_shall_be_0() {
        let section_value = SectionValue::default();
        let aggregated = AggregatedCoverage::from_section(&section_value);

        assert_aggregated_counters_eq(&aggregated.lines, 0, 0);
        assert_aggregated_counters_eq(&aggregated.functions, 0, 0);
        assert_aggregated_counters_eq(&aggregated.branches, 0, 0);
    }

    #[test]
    fn when_creating_from_a_section_with_1_line_3_hit_it_shall_be_1_1() {
        let section_value = SectionValue::default().insert_line(1, 3);

        let aggregated = AggregatedCoverage::from_section(&section_value);
        assert_aggregated_counters_eq(&aggregated.lines, 1, 1);
    }

    #[test]
    fn when_creating_from_a_section_with_1_function_3_hit_it_shall_be_1_1() {
        let section_value = SectionValue::default().insert_function("f", 3);

        let aggregated = AggregatedCoverage::from_section(&section_value);
        assert_aggregated_counters_eq(&aggregated.functions, 1, 1);
    }

    #[test]
    fn when_creating_from_a_section_with_1_branch_3_hit_it_shall_be_1_1() {
        let section_value = SectionValue::default().insert_branch(1, 3);

        let aggregated = AggregatedCoverage::from_section(&section_value);
        assert_aggregated_counters_eq(&aggregated.branches, 1, 1);
    }

    #[test]
    fn when_creating_from_a_section_with_1_line_0_hit_it_shall_be_1_0() {
        let section_value = SectionValue::default().insert_line(1, 0);

        let aggregated = AggregatedCoverage::from_section(&section_value);
        assert_aggregated_counters_eq(&aggregated.lines, 1, 0);
    }

    #[test]
    fn when_creating_from_a_section_with_1_function_0_hit_it_shall_be_1_0() {
        let section_value = SectionValue::default().insert_function("f", 0);

        let aggregated = AggregatedCoverage::from_section(&section_value);
        assert_aggregated_counters_eq(&aggregated.functions, 1, 0);
    }

    #[test]
    fn when_creating_from_a_section_with_1_branch_0_hit_it_shall_be_1_0() {
        let section_value = SectionValue::default().insert_branch(1, 0);

        let aggregated = AggregatedCoverage::from_section(&section_value);
        assert_aggregated_counters_eq(&aggregated.branches, 1, 0);
    }

    #[test]
    fn when_creating_from_a_section_with_3_lines_2_covered_it_shall_be_3_2() {
        let section_value = SectionValue::default()
            .insert_line(1, 0)
            .insert_line(2, 3)
            .insert_line(3, 1);

        let aggregated = AggregatedCoverage::from_section(&section_value);
        assert_aggregated_counters_eq(&aggregated.lines, 3, 2);
    }

    #[test]
    fn when_creating_from_a_section_with_3_functions_2_covered_it_shall_be_3_2() {
        let section_value = SectionValue::default()
            .insert_function("f1", 0)
            .insert_function("f2", 3)
            .insert_function("f3", 1);

        let aggregated = AggregatedCoverage::from_section(&section_value);
        assert_aggregated_counters_eq(&aggregated.functions, 3, 2);
    }

    #[test]
    fn when_creating_from_a_section_with_3_branches_2_covered_it_shall_be_3_2() {
        let section_value = SectionValue::default()
            .insert_branch(1, 0)
            .insert_branch(2, 3)
            .insert_branch(3, 1);

        let aggregated = AggregatedCoverage::from_section(&section_value);
        assert_aggregated_counters_eq(&aggregated.branches, 3, 2);
    }
}
