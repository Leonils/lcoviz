struct Aggregated {
    lines_count: u32,
    covered_lines_count: u32,
}

impl Aggregated {
    fn new() -> Self {
        Self {
            lines_count: 0,
            covered_lines_count: 0,
        }
    }

    fn add(&mut self, other: &Self) {
        self.lines_count += other.lines_count;
        self.covered_lines_count += other.covered_lines_count;
    }

    fn from_section(value: lcov::report::section::Value) -> Self {
        let lines_count = value.lines.len() as u32;
        let covered_lines_count = value
            .lines
            .iter()
            .filter(|(_, value)| value.count > 0)
            .count() as u32;

        Self {
            lines_count,
            covered_lines_count,
        }
    }
}

#[cfg(test)]
mod test {
    use lcov::report::section::line::{Key as LineKey, Value as LineValue};
    use lcov::report::section::{Key as SectionKey, Value as SectionValue};

    use crate::test_utils::builders::{FromCount, FromLineNumber, FromStr};

    use super::Aggregated;

    #[test]
    fn when_creating_an_aggregate_from_scratch_count_shall_be_0() {
        let aggregated = super::Aggregated::new();
        assert_eq!(aggregated.lines_count, 0);
        assert_eq!(aggregated.covered_lines_count, 0);
    }

    #[test]
    fn when_adding_an_aggregate_to_another_the_result_should_be_the_sum_of_both() {
        let mut aggregated = super::Aggregated {
            lines_count: 10,
            covered_lines_count: 5,
        };

        let other = super::Aggregated {
            lines_count: 20,
            covered_lines_count: 10,
        };

        aggregated.add(&other);

        assert_eq!(aggregated.lines_count, 30);
        assert_eq!(aggregated.covered_lines_count, 15);
        assert_eq!(other.lines_count, 20);
        assert_eq!(other.covered_lines_count, 10);
    }

    #[test]
    fn when_creating_from_an_empty_section_it_shall_be_0() {
        let section_value = SectionValue::default();

        let aggregated = Aggregated::from_section(section_value);
        assert_eq!(aggregated.lines_count, 0);
        assert_eq!(aggregated.covered_lines_count, 0);
    }

    #[test]
    fn when_creating_from_a_section_with_1_line_3_hit_it_shall_be_1_1() {
        let mut section_value = SectionValue::default();

        section_value
            .lines
            .insert(LineKey::from_line_number(1), LineValue::from_count(3));

        let aggregated = Aggregated::from_section(section_value);
        assert_eq!(aggregated.lines_count, 1);
        assert_eq!(aggregated.covered_lines_count, 1);
    }

    #[test]
    fn when_creating_from_a_section_with_1_line_0_hit_it_shall_be_1_0() {
        let mut section_value = SectionValue::default();

        section_value
            .lines
            .insert(LineKey::from_line_number(1), LineValue::from_count(0));

        let aggregated = Aggregated::from_section(section_value);
        assert_eq!(aggregated.lines_count, 1);
        assert_eq!(aggregated.covered_lines_count, 0);
    }

    #[test]
    fn when_creating_from_a_section_with_3_lines_2_covered_it_shall_be_3_2() {
        let mut section_value = SectionValue::default();

        section_value
            .lines
            .insert(LineKey::from_line_number(1), LineValue::from_count(0));

        section_value
            .lines
            .insert(LineKey::from_line_number(2), LineValue::from_count(3));

        section_value
            .lines
            .insert(LineKey::from_line_number(3), LineValue::from_count(2));

        let aggregated = Aggregated::from_section(section_value);
        assert_eq!(aggregated.lines_count, 3);
        assert_eq!(aggregated.covered_lines_count, 2);
    }
}
