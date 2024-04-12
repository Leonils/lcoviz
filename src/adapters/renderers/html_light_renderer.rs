use crate::core::{Renderer, TestedContainer, TestedFile};

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

    fn render_aggregated_counters(
        &self,
        counters: &crate::core::AggregatedCoverageCounters,
    ) -> String {
        let percentage = counters.percentage();
        let percentage_class = self.get_percentage_class(&percentage);

        format!(
            "
                <div class=\"coverage-stats {}\">{}/{}</div>
                <div class=\"coverage-stats {}\">{}</div>
            ",
            percentage_class,
            counters.covered_count,
            counters.count,
            percentage_class,
            &self.render_optional_percentage(counters.percentage()),
        )
    }

    fn render_aggregated_coverage(&self, coverage: &crate::core::AggregatedCoverage) -> String {
        format!(
            "{}{}{}",
            self.render_aggregated_counters(&coverage.lines),
            self.render_aggregated_counters(&coverage.functions),
            self.render_aggregated_counters(&coverage.branches)
        )
    }

    fn render_file_row(&self, file: &impl TestedFile) -> String {
        format!(
            "<div>
    <div class=\"file-row\"><div class=\"item-name\">{}</div>{}</div>
</div>",
            file.get_name(),
            self.render_aggregated_coverage(file.get_aggregated_coverage()),
        )
    }

    fn render_module_row(&self, module: &impl TestedContainer) -> String {
        format!(
            "<div class=\"module-div\">
    <div><div class=\"module-row\"><div class=\"item-name\">{}</div>{}</div>
    <div class=\"module-children\">
        {}
        {}
    </div>
</div>",
            module.get_name(),
            self.render_aggregated_coverage(module.get_aggregated_coverage()),
            module
                .get_container_children()
                .iter()
                .map(|module| self.render_module_row(module))
                .collect::<Vec<String>>()
                .join("\n"),
            module
                .get_code_file_children()
                .iter()
                .map(|file| self.render_file_row(file))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl Renderer for HtmlLightRenderer {
    fn render_coverage_summary(&self, root: impl crate::core::TestedContainer) -> String {
        return format!(
            "<html>
    <head>
        <title>Coverage report</title>
        <style type=\"text/css\">
            body {{
                font-family: Arial, sans-serif;
                background-color: #eee;
            }}
            h1 {{
                font-weight: 400;
                font-size: xxx-large;
            }}
            .responsive-container {{
                max-width: 1200px;
                margin: 100px auto;
            }}
            .module-div {{
                margin-top: 6px;
            }}
            .module-children {{
                margin-left: 20px;
                margin-bottom: 8px;
            }}
            .module-row, .file-row {{
                display: flex;
                justify-content: space-between;
            }}
            .module-row {{
                font-weight: bold;
                background-color: #ccc;
                border-radius: 4px;
            }}
            .file-row {{
                font-style: italic;
                background-color: #fff;
                margin-top: 2px;
                border-radius: 4px;
            }}
            .item-name {{
                flex-grow: 1;
                margin: 4px 12px;
            }}
            .coverage-stats {{
                margin: 1px;
                width: 100px;
                max-width: 100px;
                min-width: 100px;
                text-align: center;
                border-radius: 4px;
                padding: 2px;
            }}
            .percentage-0 {{ background-color: #c10000aa; color: #fff; }}
            .percentage-1 {{ background-color: #c12e00aa; color: #fff; }}
            .percentage-2 {{ background-color: #cf461baa; color: #fff; }}
            .percentage-3 {{ background-color: #eb5f1baa; color: #fff; }}
            .percentage-4 {{ background-color: #e77724aa; color: #000; }}
            .percentage-5 {{ background-color: #e7ac24aa; color: #000; }}
            .percentage-6 {{ background-color: #e7be24aa; color: #000; }}
            .percentage-7 {{ background-color: #e3e724aa; color: #000; }}
            .percentage-8 {{ background-color: #b6e724aa; color: #000; }}
            .percentage-9 {{ background-color: #6ccd24aa; color: #fff; }}
            .percentage-10 {{ background-color: #51af22aa; color: #fff; }}
            .no-coverage {{ background-color: #ddddddaa; color: #000; }}
        </style>
    </head>
    <body>
        <main class=\"responsive-container\">
            <h1>Coverage report</h1>
            {}
        </main>
    </body>
</html>",
            root.get_container_children()
                .iter()
                .map(|module| self.render_module_row(module))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }

    fn render_file_coverage_details(&self, file: impl crate::core::TestedFile) -> String {
        return format!("<html></html>",);
    }
}
