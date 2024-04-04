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
}

#[cfg(test)]
mod test {
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
}
