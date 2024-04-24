use std::include_str;

use crate::{
    core::{
        AggregatedCoverage, AggregatedCoverageCounters, LinkPayload, LinksComputer, Renderer,
        TestedContainer, TestedFile, WithPath,
    },
    file_provider::FileLinesProvider,
    html::{
        colors::{get_percentage_class, render_optional_percentage},
        components::{Div, Gauge, Img, Link, Pre, Row, Table, Text, ToHtml},
    },
};

use super::{components::chip::render_aggregated_coverage_chips, file_icon::FileIcon};

const DEFAULT_CSS: &str = include_str!("resources/html_light_renderer.css");
const GAUGE_CSS: &str = include_str!("resources/gauge.css");
const COLORS_CSS: &str = include_str!("resources/colors.css");
const MODULE_SVG: &str = include_str!("resources/module.svg");
const MODULE_MAIN_SVG: &str = include_str!("resources/module-main.svg");
const FUNCTION_COVERED_SVG: &str = include_str!("resources/function_covered.svg");
const FUNCTION_UNCOVERED_SVG: &str = include_str!("resources/function_uncovered.svg");

pub struct HtmlLightRenderer<TLinksComputer: LinksComputer> {
    links_computer: TLinksComputer,
}

impl<TLinksComputer: LinksComputer> HtmlLightRenderer<TLinksComputer> {
    pub fn new(links_computer: TLinksComputer) -> Self {
        HtmlLightRenderer { links_computer }
    }

    fn render_gauge(
        counter: &AggregatedCoverageCounters,
        name: &str,
        add_link_to_section: bool,
    ) -> Gauge {
        let link = format!("#{}", name.to_lowercase());
        Gauge::new(
            counter.percentage(),
            &format!("{} {}/{}", name, counter.covered_count, counter.count),
            if add_link_to_section {
                Some(&link)
            } else {
                None
            },
        )
    }

    fn render_gauges(&self, coverage: &AggregatedCoverage, with_link: bool) -> Div {
        Div::new()
            .with_class("gauges")
            .with_child(Self::render_gauge(&coverage.lines, "Lines", with_link))
            .with_child(Self::render_gauge(
                &coverage.functions,
                "Functions",
                with_link,
            ))
            .with_child(Self::render_gauge(&coverage.branches, "Branches", false))
    }

    fn render_aggregated_counters(&self, counters: &AggregatedCoverageCounters) -> Vec<Div> {
        let percentage = counters.percentage();
        let percentage_class = get_percentage_class("bg", &percentage);

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
        coverage: &AggregatedCoverage,
    ) -> impl Iterator<Item = Div> {
        vec![
            self.render_aggregated_counters(&coverage.lines),
            self.render_aggregated_counters(&coverage.functions),
            self.render_aggregated_counters(&coverage.branches),
        ]
        .into_iter()
        .flatten()
    }

    fn render_file_row(
        &self,
        root: &impl WithPath,
        current_page: &impl WithPath,
        file: &impl TestedFile,
    ) -> Div {
        let link = self.links_computer.get_link_to(current_page, file);
        let img_src = self.links_computer.get_link_to_resource(
            root,
            current_page,
            FileIcon::get_icon_key(file).unwrap_or_default(),
        );

        Div::new().with_child(
            Div::new()
                .with_class("file-row")
                .with_child(
                    Div::new()
                        .with_class("file-logo")
                        .with_child(Img::new(&img_src, "Rust logo")),
                )
                .with_child(
                    Div::new()
                        .with_class("item-name")
                        .with_child(Link::from_link_payload(link)),
                )
                .with_children(self.render_aggregated_coverage(file.get_aggregated_coverage())),
        )
    }

    fn render_module_row(
        &self,
        root: &impl WithPath,
        current_page: &impl WithPath,
        module: &impl TestedContainer,
    ) -> Div {
        let submodules = module
            .get_container_children()
            .map(|module| self.render_module_row(root, current_page, module));

        let files = module
            .get_code_file_children()
            .map(|file| self.render_file_row(root, current_page, file));

        Div::new().with_class("module-div").with_child(
            Div::new()
                .with_child(
                    Div::new()
                        .with_class("module-row")
                        .with_child(Div::new().with_class("item-name").with_child(
                            Link::from_link_payload(
                                self.links_computer.get_link_to(current_page, module),
                            ),
                        ))
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

    fn render_top_module_row<'a>(
        &'a self,
        root: &impl WithPath,
        current_page: &impl WithPath,
        module: &'a impl TestedContainer,
    ) -> Div<'a> {
        let module_img_href =
            self.links_computer
                .get_link_to_resource(root, current_page, "module.svg");

        let module_page_href = self.links_computer.get_link_to(current_page, module);

        let top_module_div = Div::new()
            .with_class("top-module")
            .with_child(
                Div::new()
                    .with_class("tab")
                    .with_child(Img::new(&module_img_href, "Module icon"))
                    .with_child(Link::from_child(
                        &module_page_href.link,
                        Box::new(Text::h2(module.get_name()).with_class("code-file-name")),
                    )),
            )
            .with_child(Div::new().with_class("fill"))
            .with_children(render_aggregated_coverage_chips(
                module.get_aggregated_coverage(),
            ))
            .with_child(Div::new().with_class("w-20"));

        let submodules = module
            .get_container_children()
            .map(|module| self.render_module_row(root, current_page, module));

        let files = module
            .get_code_file_children()
            .map(|file| self.render_file_row(root, current_page, file));

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

    fn render_line(line_number: u32, file: &impl TestedFile, line: String) -> Row {
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

    fn render_lines(file: &impl TestedFile, lines: Vec<String>) -> Table {
        let rows = lines
            .iter()
            .enumerate()
            .map(|(i, line)| Self::render_line(i as u32 + 1, file, line.clone()));

        Table::new().with_rows(rows)
    }

    fn render_functions(&self, root: &impl WithPath, file: &impl TestedFile) -> Div {
        let functions = file.get_functions();

        let covered_svg =
            self.links_computer
                .get_link_to_resource(root, file, "function_covered.svg");
        let uncovered_svg =
            self.links_computer
                .get_link_to_resource(root, file, "function_uncovered.svg");

        let functions = functions.map(|(name, count)| {
            Div::new()
                .with_class("function")
                .with_class(if count > 0 {
                    "function-covered"
                } else {
                    "function-uncovered"
                })
                .with_child(Img::new(
                    if count > 0 {
                        &covered_svg
                    } else {
                        &uncovered_svg
                    },
                    "Function coverage",
                ))
                .with_child(
                    Div::new()
                        .with_class("function-name")
                        .with_child(Text::new(&name)),
                )
                .with_child(Div::new().with_class("fill"))
                .with_child(
                    Div::new()
                        .with_class("function-hit")
                        .with_child(Text::new(&format!("{} calls", count))),
                )
        });

        Div::new().with_children(functions)
    }

    fn render_navigation(&self, root: &impl WithPath, file: &impl WithPath) -> Div {
        let links = self
            .links_computer
            .get_links_from_file(root, file)
            .collect::<Vec<LinkPayload>>();

        if links.is_empty() {
            return Div::new();
        }

        let links = links
            .into_iter()
            .map(|link| {
                vec![
                    Div::new()
                        .with_class("navigation-part")
                        .with_child(Link::from_link_payload(link)),
                    Div::new().with_text(" / "),
                ]
            })
            .flatten();

        Div::new()
            .with_class("navigation")
            .with_children(links)
            .with_child(
                Div::new()
                    .with_class("navigation-part")
                    .with_text(file.get_name()),
            )
    }

    fn render_layout(
        &self,
        root: &impl WithPath,
        current: &impl WithPath,
        content: String,
    ) -> String {
        return format!(
            "<html>
    <head>
        <title>Coverage report</title>
        <link rel=\"stylesheet\" type=\"text/css\" href=\"{}\">
        <link rel=\"stylesheet\" type=\"text/css\" href=\"{}\">
        <link rel=\"stylesheet\" type=\"text/css\" href=\"{}\">
    </head>
    <body>
        <main class=\"responsive-container\">
            {}
        </main>
    </body>
</html>",
            self.links_computer
                .get_link_to_resource(root, current, "html_light_renderer.css"),
            self.links_computer
                .get_link_to_resource(root, current, "gauge.css"),
            self.links_computer
                .get_link_to_resource(root, current, "colors.css"),
            content,
        );
    }

    fn get_resources_required_by_module(
        &self,
        module: &impl TestedContainer,
    ) -> impl Iterator<Item = (&str, &str)> {
        let resources_required_by_files = module
            .get_code_file_children()
            .map(|file| FileIcon::get_resources_required_by_file(file))
            .flatten()
            .into_iter();

        let resources_required_by_submodules = module
            .get_container_children()
            .map(|submodule| self.get_resources_required_by_module(submodule))
            .flatten()
            .into_iter();

        let resources = resources_required_by_files.chain(resources_required_by_submodules);
        resources.collect::<Vec<(&str, &str)>>().into_iter()
    }

    fn render_title_with_img(
        &self,
        root: &impl WithPath,
        current: &impl WithPath,
        icon_key: &str,
    ) -> Div {
        Div::new()
            .with_class("title-with-image")
            .with_child(Img::new(
                &self
                    .links_computer
                    .get_link_to_resource(root, current, icon_key),
                "File icon",
            ))
            .with_child(Text::h1(current.get_name()).with_class("code-file-name"))
    }
}

impl<TLinksComputer: LinksComputer> Renderer for HtmlLightRenderer<TLinksComputer> {
    fn render_module_coverage_details(
        &self,
        root: &impl WithPath,
        module: &impl TestedContainer,
    ) -> String {
        let top_level_code_files = module
            .get_code_file_children()
            .map(|file| self.render_file_row(root, module, file));

        let mut main = Div::new().with_child(
            Div::new()
                .with_class("top-module-card")
                .with_class("header")
                .with_child(self.render_title_with_img(root, module, "module-main.svg"))
                .with_child(self.render_navigation(root, module))
                .with_child(self.render_gauges(module.get_aggregated_coverage(), false)),
        );
        if module.get_code_file_children().count() > 0 {
            main = main.with_child(
                Div::new().with_class("top-files-card").with_child(
                    Div::new()
                        .with_class("module-children")
                        .with_children(top_level_code_files),
                ),
            )
        }
        main = main.with_children(
            module
                .get_container_children()
                .map(|submodule| self.render_top_module_row(root, module, submodule)),
        );

        return self.render_layout(root, module, main.to_html());
    }

    fn render_file_coverage_details(
        &self,
        root: &impl WithPath,
        file: &impl TestedFile,
        file_provider: &impl FileLinesProvider,
    ) -> String {
        let lines = file_provider.get_file_lines().unwrap();

        let main = Div::new()
            .with_child(
                Div::new()
                    .with_class("top-module-card")
                    .with_child(self.render_title_with_img(
                        root,
                        file,
                        FileIcon::get_icon_key(file).unwrap_or_default(),
                    ))
                    .with_child(self.render_navigation(root, file))
                    .with_child(self.render_gauges(file.get_aggregated_coverage(), true)),
            )
            .with_child(
                Div::new()
                    .with_class("details-card")
                    .with_id("lines")
                    .with_child(Text::h2("Lines"))
                    .with_child(
                        Div::new()
                            .with_class("lines")
                            .with_child(Self::render_lines(file, lines)),
                    ),
            )
            .with_child(
                Div::new()
                    .with_class("details-card")
                    .with_id("functions")
                    .with_child(Text::h2("Functions"))
                    .with_child(
                        Div::new()
                            .with_class("functions")
                            .with_child(self.render_functions(root, file)),
                    ),
            );

        self.render_layout(root, file, main.to_html())
    }

    fn get_required_resources(
        &self,
        root: &impl TestedContainer,
    ) -> impl Iterator<Item = (&str, &str)> {
        self.get_resources_required_by_module(root).chain(
            vec![
                ("html_light_renderer.css", DEFAULT_CSS),
                ("gauge.css", GAUGE_CSS),
                ("colors.css", COLORS_CSS),
                ("module.svg", MODULE_SVG),
                ("module-main.svg", MODULE_MAIN_SVG),
                ("function_covered.svg", FUNCTION_COVERED_SVG),
                ("function_uncovered.svg", FUNCTION_UNCOVERED_SVG),
            ]
            .into_iter(),
        )
    }
}
