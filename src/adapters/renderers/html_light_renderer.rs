use crate::{
    core::{Renderer, TestedContainer, TestedFile},
    html::{Div, Text, ToHtml},
};

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
                let first_digit = p.to_string().chars().next().unwrap();
                if p == 100.0 {
                    return String::from("percentage-10");
                }
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
        Div::new().with_child(
            Div::new()
                .with_class("file-row")
                .with_child(
                    Div::new()
                        .with_class("item-name")
                        .with_text(file.get_name()),
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
}

impl Renderer for HtmlLightRenderer {
    fn render_coverage_summary(&self, root: impl crate::core::TestedContainer) -> String {
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
                        .map(|file| self.render_file_row(file)), // Fix: Call render_file_row with the file argument
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

    fn render_file_coverage_details(&self, file: impl crate::core::TestedFile) -> String {
        return format!("<html></html>",);
    }
}

const DEFAULT_CSS: &str = "
    body {
        font-family: Arial, sans-serif;
        background-color: #eee;
    }
    h1 {
        font-weight: 400;
        font-size: xxx-large;
        margin: 10px 10px 10px 0;
    }
    h2 {
        font-weight: 400;
        font-size: xx-large;
        margin: 10px 40px 10px 0;
    }
    .responsive-container {
        max-width: 1200px;
        margin: 100px auto;
    }
    .module-div {
        margin-top: 6px;
    }
    .module-children {
        margin-left: 20px;
        margin-bottom: 8px;
    }
    .module-row, .file-row {
        display: flex;
        justify-content: space-between;
    }
    .module-row {
        font-weight: bold;
        background-color: #ccc;
        border-radius: 4px;
    }
    .file-row {
        font-style: italic;
        background-color: #fff;
        margin-top: 2px;
        border-radius: 4px;
    }
    .item-name {
        flex-grow: 1;
        margin: 4px 12px;
    }
    .coverage-stats {
        margin: 1px;
        width: 100px;
        max-width: 100px;
        min-width: 100px;
        text-align: center;
        border-radius: 4px;
        padding: 2px;
    }
    .header {
        margin-bottom: 60px;
    }
    .top-module-card {
        margin-top: 20px;
        border-radius: 4px;
        background-color: #fff;
        padding: 20px;
        box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
    }
    .top-module {
        display: flex;
        margin: 0 0 30px 20px;
    }
    .top-module > h2, .top-module > h1 {
        flex-grow: 1;
    }
    .coverage-stats-chip {
        border: solid 2px #555;
        margin: auto 0 auto 10px;
        border-radius: 15px;
        display: flex;
        position: relative;
    }
    .coverage-stats-chip-left {
        padding: 4px 10px;
        border-radius: 15px 0 0 15px;
        text-align: right;
        background-color: #fff;
    }
    .coverage-stats-chip-right {
        padding: 4px 10px;
        border-radius: 0 15px 15px 0;
    }
    .percentage-0 { background-color: #c10000aa; color: #fff; }
    .percentage-1 { background-color: #c12e00aa; color: #fff; }
    .percentage-2 { background-color: #cf461baa; color: #fff; }
    .percentage-3 { background-color: #eb5f1baa; color: #fff; }
    .percentage-4 { background-color: #e77724aa; color: #000; }
    .percentage-5 { background-color: #e7ac24aa; color: #000; }
    .percentage-6 { background-color: #e7be24aa; color: #000; }
    .percentage-7 { background-color: #e3e724aa; color: #000; }
    .percentage-8 { background-color: #b6e724aa; color: #000; }
    .percentage-9 { background-color: #6ccd24aa; color: #fff; }
    .percentage-10 { background-color: #51af22aa; color: #fff; }
    .no-coverage { background-color: #ddddddaa; color: #000; }
    .percentage-0-chip { border-color: #c10000aa; }
    .percentage-1-chip { border-color: #c12e00aa; }
    .percentage-2-chip { border-color: #cf461baa; }
    .percentage-3-chip { border-color: #eb5f1baa; }
    .percentage-4-chip { border-color: #e77724aa; }
    .percentage-5-chip { border-color: #e7ac24aa; }
    .percentage-6-chip { border-color: #e7be24aa; }
    .percentage-7-chip { border-color: #e3e724aa; }
    .percentage-8-chip { border-color: #b6e724aa; }
    .percentage-9-chip { border-color: #6ccd24aa; }
    .percentage-10-chip { border-color: #51af22aa; }
    .no-coverage-chip { border-color: #ddddddaa; }
";
