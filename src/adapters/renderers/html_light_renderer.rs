use std::include_str;
use std::path::PathBuf;

use crate::{
    adapters::renderers::common::render_optional_percentage,
    core::{LinksComputer, Renderer, TestedContainer, TestedFile, WithPath},
    file_provider::FileLinesProvider,
    html::{Div, Img, Link, Text, ToHtml},
};

use super::common::get_percentage_class;

const DEFAULT_CSS: &str = include_str!("resources/html_light_renderer.css");

pub struct HtmlLightRenderer<TLinksComputer: LinksComputer> {
    links_computer: TLinksComputer,
}

impl<TLinksComputer: LinksComputer> HtmlLightRenderer<TLinksComputer> {
    pub fn new(links_computer: TLinksComputer) -> Self {
        HtmlLightRenderer { links_computer }
    }

    fn render_aggregated_counter_chip(
        &self,
        name: &str,
        counter: &crate::core::AggregatedCoverageCounters,
    ) -> Div {
        let percentage = counter.percentage();
        let percentage_class = get_percentage_class("percentage", &percentage);
        let percentage_chip_class = get_percentage_class("chip", &percentage);

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
                    .with_text(&render_optional_percentage(percentage)),
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
        let percentage_class = get_percentage_class("percentage", &percentage);

        vec![
            Div::new()
                .with_class("coverage-stats")
                .with_class(percentage_class.as_str())
                .with_text(&format!("{}/{}", counters.covered_count, counters.count)),
            Div::new()
                .with_class("coverage-stats")
                .with_class(percentage_class.as_str())
                .with_text(&render_optional_percentage(percentage)),
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

    fn render_file_row(&self, root: &impl WithPath, file: &impl TestedFile) -> Div {
        let file_path = file.get_path();
        let file_extension = file_path.extension().unwrap_or_default();
        let file_target = PathBuf::new()
            .join(file.get_path_relative_to(&root.get_path()))
            .with_extension(format!("{}.html", file_extension.to_string_lossy()));

        Div::new().with_child(
            Div::new()
                .with_class("file-row")
                .with_child(
                    Div::new()
                        .with_class("file-logo")
                        .with_child(Img::new("https://raw.githubusercontent.com/rust-lang/rust-artwork/master/logo/rust-logo-blk.svg", "Rust logo")),
                )
                .with_child(
                    Div::new()
                        .with_class("item-name")
                        .with_child(Link::new(file_target.to_str().unwrap(), file.get_name())),
                )
                .with_children(self.render_aggregated_coverage(file.get_aggregated_coverage())),
        )
    }

    fn render_module_row(&self, root: &impl WithPath, module: &impl TestedContainer) -> Div {
        let submodules = module
            .get_container_children()
            .map(|module| self.render_module_row(root, module));

        let files = module
            .get_code_file_children()
            .map(|file| self.render_file_row(root, file));

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

    fn render_top_module_row(&self, root: &impl WithPath, module: &impl TestedContainer) -> Div {
        let top_module_div = Div::new()
            .with_class("top-module")
            .with_child(Text::h2(module.get_name()))
            .with_children(self.render_aggregated_coverage_chips(module.get_aggregated_coverage()));

        let submodules = module
            .get_container_children()
            .map(|module| self.render_module_row(root, module));

        let files = module
            .get_code_file_children()
            .map(|file| self.render_file_row(root, file));

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

    fn render_navigation(&self, root: &impl WithPath, file: &impl WithPath) -> Div {
        let links: Vec<Link> = self
            .links_computer
            .get_links_from_file(root, file)
            .map(|link| Link::new(&link.link, &link.text))
            .collect();

        if links.len() == 0 {
            return Div::new();
        }

        let mut nav_bar = Div::new().with_class("navigation");
        for link in links {
            nav_bar = nav_bar
                .with_child(Div::new().with_class("navigation-part").with_child(link))
                .with_child(Div::new().with_text(" / "))
        }
        nav_bar.with_child(
            Div::new()
                .with_class("navigation-part")
                .with_text(file.get_name()),
        )
    }
}

impl<TLinksComputer: LinksComputer> Renderer for HtmlLightRenderer<TLinksComputer> {
    fn render_module_coverage_details(
        &self,
        root: &impl WithPath,
        module: &impl TestedContainer,
    ) -> String {
        let root_top_module_div = Div::new()
            .with_class("top-module")
            .with_child(Text::h1(module.get_name()))
            .with_children(self.render_aggregated_coverage_chips(module.get_aggregated_coverage()));

        let top_level_code_files = Div::new()
            .with_class("top-module-card")
            .with_child(
                Div::new()
                    .with_class("top-module")
                    .with_child(Text::h2("Top level code files")),
            )
            .with_child(
                Div::new().with_class("module-children").with_children(
                    module
                        .get_code_file_children()
                        .map(|file| self.render_file_row(module, file)),
                ),
            );

        let main = Div::new()
            .with_child(
                Div::new()
                    .with_class("top-module-card")
                    .with_class("header")
                    .with_child(root_top_module_div)
                    .with_child(self.render_navigation(root, module)),
            )
            .with_children(
                module
                    .get_container_children()
                    .map(|submodule| self.render_top_module_row(module, submodule)),
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
        root: &impl WithPath,
        file: &impl crate::core::TestedFile,
        file_provider: &impl FileLinesProvider,
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
                {}
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
            self.render_navigation(root, file).to_html(),
            self.render_lines(file, lines)
        );
    }
}
