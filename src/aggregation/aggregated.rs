#[derive(Debug, PartialEq, Default)]
pub struct Aggregated {
    pub lines_count: u32,
    pub covered_lines_count: u32,
    pub functions_count: u32,
    pub covered_functions_count: u32,
    pub branches_count: u32,
    pub covered_branches_count: u32,
}

impl Aggregated {
    fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, other: &Self) {
        self.lines_count += other.lines_count;
        self.covered_lines_count += other.covered_lines_count;
        self.functions_count += other.functions_count;
        self.covered_functions_count += other.covered_functions_count;
        self.branches_count += other.branches_count;
        self.covered_branches_count += other.covered_branches_count;
    }

    pub fn from_section(value: lcov::report::section::Value) -> Self {
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
            lines_count,
            covered_lines_count,
            functions_count,
            covered_functions_count,
            branches_count,
            covered_branches_count,
        }
    }
}

#[cfg(test)]
pub fn assert_aggregate_eq(aggregated: &Aggregated, lines_count: u32, covered_lines_count: u32) {
    assert_eq!(aggregated.lines_count, lines_count);
    assert_eq!(aggregated.covered_lines_count, covered_lines_count);
}

#[cfg(test)]
mod test {
    use lcov::report::section::Value as SectionValue;

    use crate::test_utils::builders::{InsertBranch, InsertFunction, InsertLine};

    use super::Aggregated;

    #[test]
    fn when_creating_an_aggregate_from_scratch_line_count_shall_be_0() {
        let aggregated = super::Aggregated::new();
        assert_eq!(aggregated.lines_count, 0);
        assert_eq!(aggregated.covered_lines_count, 0);
    }

    #[test]
    fn when_creating_an_aggregate_from_scratch_function_count_shall_be_0() {
        let aggregated = super::Aggregated::new();
        assert_eq!(aggregated.functions_count, 0);
        assert_eq!(aggregated.covered_functions_count, 0);
    }

    #[test]
    fn when_creating_an_aggregate_from_scratch_branch_count_shall_be_0() {
        let aggregated = super::Aggregated::new();
        assert_eq!(aggregated.branches_count, 0);
        assert_eq!(aggregated.covered_branches_count, 0);
    }

    #[test]
    fn when_adding_an_aggregate_to_another_the_lines_result_should_be_the_sum_of_both() {
        let mut aggregated = super::Aggregated {
            lines_count: 10,
            covered_lines_count: 5,
            ..Default::default()
        };

        let other = super::Aggregated {
            lines_count: 20,
            covered_lines_count: 10,
            ..Default::default()
        };

        aggregated.add(&other);

        assert_eq!(aggregated.lines_count, 30);
        assert_eq!(aggregated.covered_lines_count, 15);
        assert_eq!(other.lines_count, 20);
        assert_eq!(other.covered_lines_count, 10);
    }

    #[test]
    fn when_adding_an_aggregate_to_another_the_functions_result_should_be_the_sum_of_both() {
        let mut aggregated = super::Aggregated {
            functions_count: 10,
            covered_functions_count: 5,
            ..Default::default()
        };

        let other = super::Aggregated {
            functions_count: 20,
            covered_functions_count: 10,
            ..Default::default()
        };

        aggregated.add(&other);

        assert_eq!(aggregated.functions_count, 30);
        assert_eq!(aggregated.covered_functions_count, 15);
        assert_eq!(other.functions_count, 20);
        assert_eq!(other.covered_functions_count, 10);
    }

    #[test]
    fn when_adding_an_aggregate_to_another_the_branches_result_should_be_the_sum_of_both() {
        let mut aggregated = super::Aggregated {
            branches_count: 10,
            covered_branches_count: 5,
            ..Default::default()
        };

        let other = super::Aggregated {
            branches_count: 20,
            covered_branches_count: 10,
            ..Default::default()
        };

        aggregated.add(&other);

        assert_eq!(aggregated.branches_count, 30);
        assert_eq!(aggregated.covered_branches_count, 15);
        assert_eq!(other.branches_count, 20);
        assert_eq!(other.covered_branches_count, 10);
    }

    #[test]
    fn when_creating_from_an_empty_section_line_counts_shall_be_0() {
        let section_value = SectionValue::default();

        let aggregated = Aggregated::from_section(section_value);
        assert_eq!(aggregated.lines_count, 0);
        assert_eq!(aggregated.covered_lines_count, 0);
    }

    #[test]
    fn when_creating_from_an_empty_section_function_counts_shall_be_0() {
        let section_value = SectionValue::default();

        let aggregated = Aggregated::from_section(section_value);
        assert_eq!(aggregated.functions_count, 0);
        assert_eq!(aggregated.covered_functions_count, 0);
    }

    #[test]
    fn when_creating_from_an_empty_section_branch_counts_shall_be_0() {
        let section_value = SectionValue::default();

        let aggregated = Aggregated::from_section(section_value);
        assert_eq!(aggregated.branches_count, 0);
        assert_eq!(aggregated.covered_branches_count, 0);
    }

    #[test]
    fn when_creating_from_a_section_with_1_line_3_hit_it_shall_be_1_1() {
        let mut section_value = SectionValue::default();
        section_value.lines.insert_line(1, 3);

        let aggregated = Aggregated::from_section(section_value);
        assert_eq!(aggregated.lines_count, 1);
        assert_eq!(aggregated.covered_lines_count, 1);
    }

    #[test]
    fn when_creating_from_a_section_with_1_function_3_hit_it_shall_be_1_1() {
        let mut section_value = SectionValue::default();
        section_value.functions.insert_function("f", 3);

        let aggregated = Aggregated::from_section(section_value);
        assert_eq!(aggregated.functions_count, 1);
        assert_eq!(aggregated.covered_functions_count, 1);
    }

    #[test]
    fn when_creating_from_a_section_with_1_branch_3_hit_it_shall_be_1_1() {
        let mut section_value = SectionValue::default();
        section_value.branches.insert_branch(1, 3);

        let aggregated = Aggregated::from_section(section_value);
        assert_eq!(aggregated.branches_count, 1);
        assert_eq!(aggregated.covered_branches_count, 1);
    }

    #[test]
    fn when_creating_from_a_section_with_1_line_0_hit_it_shall_be_1_0() {
        let mut section_value = SectionValue::default();
        section_value.lines.insert_line(1, 0);

        let aggregated = Aggregated::from_section(section_value);
        assert_eq!(aggregated.lines_count, 1);
        assert_eq!(aggregated.covered_lines_count, 0);
    }

    #[test]
    fn when_creating_from_a_section_with_1_function_0_hit_it_shall_be_1_0() {
        let mut section_value = SectionValue::default();
        section_value.functions.insert_function("f", 0);

        let aggregated = Aggregated::from_section(section_value);
        assert_eq!(aggregated.functions_count, 1);
        assert_eq!(aggregated.covered_functions_count, 0);
    }

    #[test]
    fn when_creating_from_a_section_with_1_branch_0_hit_it_shall_be_1_0() {
        let mut section_value = SectionValue::default();
        section_value.branches.insert_branch(1, 0);

        let aggregated = Aggregated::from_section(section_value);
        assert_eq!(aggregated.branches_count, 1);
        assert_eq!(aggregated.covered_branches_count, 0);
    }

    #[test]
    fn when_creating_from_a_section_with_3_lines_2_covered_it_shall_be_3_2() {
        let mut section_value = SectionValue::default();
        section_value
            .lines
            .insert_line(1, 0)
            .insert_line(2, 3)
            .insert_line(3, 1);

        let aggregated = Aggregated::from_section(section_value);
        assert_eq!(aggregated.lines_count, 3);
        assert_eq!(aggregated.covered_lines_count, 2);
    }

    #[test]
    fn when_creating_from_a_section_with_3_functions_2_covered_it_shall_be_3_2() {
        let mut section_value = SectionValue::default();
        section_value
            .functions
            .insert_function("f1", 0)
            .insert_function("f2", 3)
            .insert_function("f3", 1);

        let aggregated = Aggregated::from_section(section_value);
        assert_eq!(aggregated.functions_count, 3);
        assert_eq!(aggregated.covered_functions_count, 2);
    }

    #[test]
    fn when_creating_from_a_section_with_3_branches_2_covered_it_shall_be_3_2() {
        let mut section_value = SectionValue::default();
        section_value
            .branches
            .insert_branch(1, 0)
            .insert_branch(2, 3)
            .insert_branch(3, 1);

        let aggregated = Aggregated::from_section(section_value);
        assert_eq!(aggregated.branches_count, 3);
        assert_eq!(aggregated.covered_branches_count, 2);
    }
}
