use std::include_str;
use std::path::PathBuf;

use crate::{
    core::{Renderer, TestedContainer, TestedFile},
    file_provider::FileLinesProvider,
    html::{Div, Link, Text, ToHtml},
};

const DEFAULT_CSS: &str = include_str!("resources/html_light_renderer.css");

pub struct HtmlLightRenderer {
    // ...
}

impl HtmlLightRenderer {
    fn render_optional_percentage(&self, percentage: Option<f32>) -> String {
        percentage
            .map(|p| format!("{:.2}%", p))
            .unwrap_or("-".to_string())
    }

    fn get_percentage_class(&self, percentage: &Option<f32>) -> String {
        percentage
            .map(|p| {
                if p == 100.0 {
                    return String::from("percentage-10");
                }
                let first_digit = p.to_string().chars().next().unwrap();
                return format!("percentage-{}", first_digit);
            })
            .unwrap_or(String::from("no-coverage"))
    }

    fn render_aggregated_counter_chip(
        &self,
        name: &str,
        counter: &crate::core::AggregatedCoverageCounters,
    ) -> Div {
        let percentage = counter.percentage();
        let percentage_class = self.get_percentage_class(&percentage);
        let percentage_chip_class = format!("{}-chip", percentage_class);

        let div = Div::new()
            .with_class("coverage-stats-chip")
            .with_class(percentage_chip_class.as_str())
            .with_child(
                Div::new()
                    .with_class("coverage-stats-chip-left")
                    .with_text(&format!(
                        "{} {}/{}",
                        name, counter.covered_count, counter.count
                    )),
            )
            .with_child(
                Div::new()
                    .with_class("coverage-stats-chip-right")
                    .with_class(&percentage_class)
                    .with_text(&self.render_optional_percentage(percentage)),
            );

        div
    }

    fn render_aggregated_coverage_chips(
        &self,
        coverage: &crate::core::AggregatedCoverage,
    ) -> impl Iterator<Item = Div> {
        vec![
            self.render_aggregated_counter_chip("lines", &coverage.lines),
            self.render_aggregated_counter_chip("functions", &coverage.functions),
            self.render_aggregated_counter_chip("branches", &coverage.branches),
        ]
        .into_iter()
    }

    fn render_aggregated_counters(
        &self,
        counters: &crate::core::AggregatedCoverageCounters,
    ) -> Vec<Div> {
        let percentage = counters.percentage();
        let percentage_class = self.get_percentage_class(&percentage);

        vec![
            Div::new()
                .with_class("coverage-stats")
                .with_class(percentage_class.as_str())
                .with_text(&format!("{}/{}", counters.covered_count, counters.count)),
            Div::new()
                .with_class("coverage-stats")
                .with_class(percentage_class.as_str())
                .with_text(&self.render_optional_percentage(percentage)),
        ]
    }

    fn render_aggregated_coverage(
        &self,
        coverage: &crate::core::AggregatedCoverage,
    ) -> impl Iterator<Item = Div> {
        vec![
            self.render_aggregated_counters(&coverage.lines),
            self.render_aggregated_counters(&coverage.functions),
            self.render_aggregated_counters(&coverage.branches),
        ]
        .into_iter()
        .flatten()
    }

    fn render_file_row(&self, file: &impl TestedFile) -> Div {
        let mut file_target = PathBuf::new()
            .join("details")
            .join(file.get_path_relative_to_prefix());
        file_target.set_extension("html");

        Div::new().with_child(
            Div::new()
                .with_class("file-row")
                .with_child(
                    Div::new()
                        .with_class("item-name")
                        .with_child(Link::new(file_target.to_str().unwrap(), file.get_name())),
                )
                .with_children(self.render_aggregated_coverage(file.get_aggregated_coverage())),
        )
    }

    fn render_module_row(&self, module: &impl TestedContainer) -> Div {
        let submodules = module
            .get_container_children()
            .map(|module| self.render_module_row(module));

        let files = module
            .get_code_file_children()
            .map(|file| self.render_file_row(file));

        Div::new().with_class("module-div").with_child(
            Div::new()
                .with_child(
                    Div::new()
                        .with_class("module-row")
                        .with_child(
                            Div::new()
                                .with_class("item-name")
                                .with_text(module.get_name()),
                        )
                        .with_children(
                            self.render_aggregated_coverage(module.get_aggregated_coverage()),
                        ),
                )
                .with_child(
                    Div::new()
                        .with_class("module-children")
                        .with_children(submodules)
                        .with_children(files),
                ),
        )
    }

    fn render_top_module_row(&self, module: &impl TestedContainer) -> Div {
        let top_module_div = Div::new()
            .with_class("top-module")
            .with_child(Text::h2(module.get_name()))
            .with_children(self.render_aggregated_coverage_chips(module.get_aggregated_coverage()));

        let submodules = module
            .get_container_children()
            .map(|module| self.render_module_row(module));

        let files = module
            .get_code_file_children()
            .map(|file| self.render_file_row(file));

        Div::new()
            .with_class("top-module-card")
            .with_child(top_module_div)
            .with_child(
                Div::new()
                    .with_class("module-children")
                    .with_children(submodules)
                    .with_children(files),
            )
    }

    fn render_lines(&self, file: &impl crate::core::TestedFile, lines: Vec<String>) -> String {
        let mut result = String::new();
        for (i, line) in lines.iter().enumerate() {
            let line_number = i + 1;
            let coverage = file.get_line_coverage(line_number as u32);

            let line_div = match coverage {
                Some(cov) if cov > 0 => Div::new()
                    .with_class("line-covered")
                    .with_text(&format!("{:4} | {:4} | {}", line_number, cov, line)),
                Some(cov) => Div::new()
                    .with_class("line-not-covered")
                    .with_text(&format!("{:4} | {:4} | {}", line_number, cov, line)),
                None => Div::new()
                    .with_class("line-not-tested")
                    .with_text(&format!("{:4} |      | {}", line_number, line)),
            };

            result.push_str(&line_div.to_html());
        }
        return result;
    }
}

impl Renderer for HtmlLightRenderer {
    fn render_coverage_summary(&self, root: &impl crate::core::TestedContainer) -> String {
        let root_top_module_div = Div::new()
            .with_class("top-module")
            .with_child(Text::h1("Coverage report"))
            .with_children(self.render_aggregated_coverage_chips(root.get_aggregated_coverage()));

        let top_level_code_files = Div::new()
            .with_class("top-module-card")
            .with_child(
                Div::new()
                    .with_class("top-module")
                    .with_child(Text::h2("Top level code files")),
            )
            .with_child(
                Div::new().with_class("module-children").with_children(
                    root.get_code_file_children()
                        .map(|file| self.render_file_row(file)),
                ),
            );

        let main = Div::new()
            .with_child(
                Div::new()
                    .with_class("top-module-card")
                    .with_class("header")
                    .with_child(root_top_module_div),
            )
            .with_children(
                root.get_container_children()
                    .map(|module| self.render_top_module_row(module)),
            )
            .with_child(top_level_code_files);

        return format!(
            "<html>
    <head>
        <title>Coverage report</title>
        <style type=\"text/css\">
            {}
        </style>
    </head>
    <body>
        <main class=\"responsive-container\">
            {}
        </main>
    </body>
</html>",
            DEFAULT_CSS,
            main.to_html()
        );
    }

    fn render_file_coverage_details(
        &self,
        file: &impl crate::core::TestedFile,
        file_provider: impl FileLinesProvider,
    ) -> String {
        let lines = file_provider.get_file_lines().unwrap();
        return format!(
            "<html>
    <head>
        <title>Coverage report</title>
        <style type=\"text/css\">
            {}
        </style>
    </head>
    <body>
        <main class=\"responsive-container\">
            <div class=\"top-module-card\">
                <div class=\"top-module\">
                    <h1>File: {}</h1>
                    {}
                </div>
            </div>
            <div class=\"top-module-card\">
                <h2>Lines</h2>
                <pre>{}</pre>
            </div>
        </main>
    </body>
</html>",
            DEFAULT_CSS,
            file.get_name(),
            self.render_aggregated_coverage_chips(file.get_aggregated_coverage())
                .map(|chip| chip.to_html())
                .collect::<String>(),
            self.render_lines(file, lines)
        );
    }
}
