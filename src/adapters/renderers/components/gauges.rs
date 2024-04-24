use crate::{
    core::{AggregatedCoverage, AggregatedCoverageCounters},
    html::components::{Div, Gauge, ToHtml},
};

pub struct CoverageGauges<'a> {
    coverage: &'a AggregatedCoverage,
    with_link: bool,
}

impl<'a> CoverageGauges<'a> {
    pub fn new(coverage: &'a AggregatedCoverage, with_link: bool) -> Self {
        Self {
            coverage,
            with_link,
        }
    }

    fn render_gauge(counter: &AggregatedCoverageCounters, name: &str, with_link: bool) -> Gauge {
        let link = format!("#{}", name.to_lowercase());
        Gauge::new(
            counter.percentage(),
            &format!("{} {}/{}", name, counter.covered_count, counter.count),
            if with_link { Some(&link) } else { None },
        )
    }

    fn render_gauges(&self) -> Div {
        Div::new()
            .with_class("gauges")
            .with_child(Self::render_gauge(
                &self.coverage.lines,
                "Lines",
                self.with_link,
            ))
            .with_child(Self::render_gauge(
                &self.coverage.functions,
                "Functions",
                self.with_link,
            ))
            .with_child(Self::render_gauge(
                &self.coverage.branches,
                "Branches",
                false,
            ))
    }
}

impl<'a> ToHtml for CoverageGauges<'a> {
    fn to_html(&self) -> String {
        self.render_gauges().to_html()
    }
}

#[cfg(test)]
mod tests {
    use crate::adapters::renderers::components::gauges::CoverageGauges;
    use crate::core::AggregatedCoverageCounters;
    use crate::html::components::ToHtml;

    #[test]
    fn test_coverage_gauges() {
        let coverage = crate::core::AggregatedCoverage {
            lines: AggregatedCoverageCounters::new(10, 5),
            functions: AggregatedCoverageCounters::new(10, 4),
            branches: AggregatedCoverageCounters::new(10, 3),
        };
        let gauges = CoverageGauges::new(&coverage, true);
        let html = gauges.to_html();

        assert!(
            html.contains(
                r##"<div class="gauge-c bg-5" style="transform: rotate(0.25turn)"></div><div class="gauge-data"><span class="percent">50.00%</span></div></div><div><a href="#lines">Lines 5/10</a></div>"##,
            ),
        );

        assert!(
            html.contains(
                r##"<div class="gauge-c bg-4" style="transform: rotate(0.20turn)"></div><div class="gauge-data"><span class="percent">40.00%</span></div></div><div><a href="#functions">Functions 4/10</a></div>"##,
            ),
        );

        assert!(
            html.contains(
                r##"<div class="gauge-c bg-3" style="transform: rotate(0.15turn)"></div><div class="gauge-data"><span class="percent">30.00%</span></div></div><div>Branches 3/10</div>"##,
            ),
        );
    }
}
