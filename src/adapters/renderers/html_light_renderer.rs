use std::include_str;

use crate::{
    adapters::renderers::common::render_optional_percentage,
    core::{LinksComputer, Renderer, TestedContainer, TestedFile, WithPath},
    file_provider::FileLinesProvider,
    html::components::{Div, Img, Link, Pre, Row, Table, Text, ToHtml},
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
        let link = self.links_computer.get_link_to(root, file);

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
                        .with_child(Link::from_link_payload(link))
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
            .with_child(Div::new().with_class("fill"))
            .with_children(self.render_aggregated_coverage_chips(module.get_aggregated_coverage()))
            .with_child(Div::new().with_class("w-20"));

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

    fn render_line(line_number: u32, file: &impl crate::core::TestedFile, line: String) -> Row {
        let coverage = file.get_line_coverage(line_number as u32);

        let class = match coverage {
            Some(cov) if cov > 0 => "line-covered",
            Some(_) => "line-not-covered",
            None => "line-not-tested",
        };

        Row::new()
            .with_class(class)
            .with_cell(Text::new(&line_number.to_string()))
            .with_cell(Text::new(
                &coverage.map(|c| c.to_string()).unwrap_or_default(),
            ))
            .with_cell(Pre::new(&line))
    }

    fn render_lines(file: &impl crate::core::TestedFile, lines: Vec<String>) -> Table {
        let rows = lines
            .iter()
            .enumerate()
            .map(|(i, line)| Self::render_line(i as u32 + 1, file, line.clone()));

        Table::new().with_rows(rows)
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

    fn render_layout(content: String) -> String {
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
            DEFAULT_CSS, content,
        );
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
            .with_child(Div::new().with_class("fill"))
            .with_children(self.render_aggregated_coverage_chips(module.get_aggregated_coverage()))
            .with_child(Div::new().with_class("w-20"));

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
                    .with_child(Text::h1(module.get_name()))
                    .with_child(self.render_navigation(root, module)),
            )
            .with_children(
                module
                    .get_container_children()
                    .map(|submodule| self.render_top_module_row(module, submodule)),
            )
            .with_child(top_level_code_files);

        return Self::render_layout(main.to_html());
    }

    fn render_file_coverage_details(
        &self,
        root: &impl WithPath,
        file: &impl crate::core::TestedFile,
        file_provider: &impl FileLinesProvider,
    ) -> String {
        let lines = file_provider.get_file_lines().unwrap();
        let main =
            Div::new()
                .with_child(
                    Div::new()
                        .with_class("top-module-card")
                        .with_child(
                            Div::new()
                                .with_class("top-module")
                                .with_child(Div::new().with_class("fill"))
                                .with_children(self.render_aggregated_coverage_chips(
                                    file.get_aggregated_coverage(),
                                ))
                                .with_child(Div::new().with_class("w-20")),
                        )
                        .with_child(Text::h1(file.get_name()))
                        .with_child(self.render_navigation(root, file)),
                )
                .with_child(
                    Div::new()
                        .with_class("top-module-card")
                        .with_child(Text::h2("Lines"))
                        .with_child(
                            Div::new()
                                .with_class("lines")
                                .with_child(Self::render_lines(file, lines)),
                        ),
                );

        Self::render_layout(main.to_html())
    }
}
