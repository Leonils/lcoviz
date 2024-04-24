use crate::{
    core::{AggregatedCoverage, AggregatedCoverageCounters},
    html::{
        colors::{get_percentage_class, render_optional_percentage},
        components::{Div, ToHtml},
    },
};

pub struct CoverageChip<'a> {
    name: &'a str,
    counter: &'a AggregatedCoverageCounters,
}
impl<'a> CoverageChip<'a> {
    pub fn new(name: &'a str, counter: &'a AggregatedCoverageCounters) -> Self {
        Self { name, counter }
    }
}
impl<'a> ToHtml for CoverageChip<'a> {
    fn to_html(&self) -> String {
        let percentage = self.counter.percentage();
        let percentage_class = get_percentage_class("bg", &percentage);
        let percentage_chip_class = get_percentage_class("border", &percentage);

        let div = Div::new()
            .with_class("coverage-stats-chip")
            .with_class(percentage_chip_class.as_str())
            .with_child(
                Div::new()
                    .with_class("coverage-stats-chip-left")
                    .with_text(&format!(
                        "{} {}/{}",
                        self.name, self.counter.covered_count, self.counter.count
                    )),
            )
            .with_child(
                Div::new()
                    .with_class("coverage-stats-chip-right")
                    .with_class(&percentage_class)
                    .with_text(&render_optional_percentage(percentage)),
            );

        div.to_html()
    }
}

pub fn render_aggregated_coverage_chips(
    coverage: &AggregatedCoverage,
) -> impl Iterator<Item = CoverageChip> {
    vec![
        CoverageChip::new("Lines", &coverage.lines),
        CoverageChip::new("Functions", &coverage.functions),
        CoverageChip::new("Branches", &coverage.branches),
    ]
    .into_iter()
}

#[cfg(test)]
mod tests {
    use crate::core::AggregatedCoverageCounters;

    use super::*;

    #[test]
    fn when_rendering_a_single_chip_it_shall_have_all_infos() {
        let counter = AggregatedCoverageCounters::new(10, 5);
        let chip = CoverageChip::new("Lines", &counter);
        let html = chip.to_html();

        assert!(html.contains("Lines 5/10"));
        assert!(html.contains("50.00%"));
    }

    #[test]
    fn when_rendering_multiple_chips_it_shall_have_all_infos() {
        let coverage = AggregatedCoverage {
            lines: AggregatedCoverageCounters::new(10, 5),
            functions: AggregatedCoverageCounters::new(10, 4),
            branches: AggregatedCoverageCounters::new(10, 3),
        };

        let chips = render_aggregated_coverage_chips(&coverage).collect::<Vec<_>>();
        assert_eq!(chips.len(), 3);

        let html = chips.iter().map(|chip| chip.to_html()).collect::<Vec<_>>();
        assert!(html[0].contains("Lines 5/10"));
        assert!(html[0].contains("50.00%"));
        assert!(html[1].contains("Functions 4/10"));
        assert!(html[1].contains("40.00%"));
        assert!(html[2].contains("Branches 3/10"));
        assert!(html[2].contains("30.00%"));
    }
}
