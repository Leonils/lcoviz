use crate::{
    core::{
        AggregatedCoverage, AggregatedCoverageCounters, FileLinesProvider, Renderer,
        TestedContainer, TestedFile, WithPath,
    },
    html::colors::render_optional_percentage,
};

pub struct TextSinglePageRenderer;

impl TextSinglePageRenderer {
    fn render_aggregated_counters(counters: &AggregatedCoverageCounters) -> String {
        let counters_string = format!("{}/{}", counters.covered_count, counters.count);
        format!(
            "{: >10} {: >8}",
            counters_string,
            render_optional_percentage(counters.percentage()),
        )
    }

    fn render_aggregated_coverage(coverage: &AggregatedCoverage) -> String {
        format!(
            "Lines {}    Functions {}    Branches {}",
            Self::render_aggregated_counters(&coverage.lines),
            Self::render_aggregated_counters(&coverage.functions),
            Self::render_aggregated_counters(&coverage.branches),
        )
    }

    fn render_line(level: u32, name: &str, coverage: &AggregatedCoverage) -> String {
        let name_wit_padding = "  ".repeat(level as usize) + name;
        format!(
            "{: <50} {}
",
            name_wit_padding,
            Self::render_aggregated_coverage(coverage)
        )
    }

    fn render_module(module: &impl TestedContainer, level: u32) -> String {
        let mut output = String::new();

        for file in module.get_code_file_children() {
            output.push_str(&Self::render_line(
                level,
                file.get_name(),
                file.get_aggregated_coverage(),
            ));
        }
        for submodule in module.get_container_children() {
            output.push_str(&Self::render_line(
                level,
                submodule.get_name(),
                submodule.get_aggregated_coverage(),
            ));
            output.push_str(&Self::render_module(submodule, level + 1));
        }
        output
    }

    fn render_root(root: &impl TestedContainer) -> String {
        format!(
            r#"{}:
  - Lines     {}
  - Functions {}
  - Branches  {}

Details:
"#,
            root.get_name(),
            Self::render_aggregated_counters(&root.get_aggregated_coverage().lines),
            Self::render_aggregated_counters(&root.get_aggregated_coverage().functions),
            Self::render_aggregated_counters(&root.get_aggregated_coverage().branches)
        )
    }
}

impl Renderer for TextSinglePageRenderer {
    fn get_required_resources(
        &self,
        _root: &impl TestedContainer,
    ) -> impl Iterator<Item = (&str, &str)> {
        [].into_iter()
    }

    fn render_file_coverage_details(
        &self,
        _root: &impl WithPath,
        _file: &impl TestedFile,
        _file_provider: &impl FileLinesProvider,
    ) -> String {
        unimplemented!("Text renderer only display a summary of the coverage")
    }

    fn render_module_coverage_details(
        &self,
        _root: &impl WithPath,
        module: &impl TestedContainer,
    ) -> String {
        format!(
            "{}
{}",
            Self::render_root(module),
            Self::render_module(module, 1)
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::aggregation::fixtures::AggregatedFixtures;

    #[test]
    fn when_rendering_simple_report_module_coverage_it_shall_display_all_modules_and_files() {
        let report = AggregatedFixtures::get_simple_full_report();
        let renderer = TextSinglePageRenderer;
        let rendered = renderer.render_module_coverage_details(&report, &report);
        assert_eq!(
            rendered,
            r#"Test report:
  - Lines            5/6   83.33%
  - Functions        3/3  100.00%
  - Branches         1/2   50.00%

Details:

  main.cpp                                         Lines        3/4   75.00%    Functions        2/2  100.00%    Branches        1/2   50.00%
  module                                           Lines        2/2  100.00%    Functions        1/1  100.00%    Branches        0/0        -
    nested.cpp                                     Lines        2/2  100.00%    Functions        1/1  100.00%    Branches        0/0        -
"#
        );
    }
}
