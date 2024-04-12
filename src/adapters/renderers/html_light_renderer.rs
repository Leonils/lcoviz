use crate::core::{Renderer, TestedContainer, TestedFile};

pub struct HtmlLightRenderer {
    // ...
}

impl HtmlLightRenderer {
    fn render_aggregated_coverage(&self, coverage: &crate::core::AggregatedCoverage) -> String {
        format!(
            "
                <div class=\"coverage-stats\">{}/{}</div>
                <div class=\"coverage-stats\">{}/{}</div>
                <div class=\"coverage-stats\">{}/{}</div>
            ",
            coverage.lines.covered_count,
            coverage.lines.count,
            coverage.functions.covered_count,
            coverage.functions.count,
            coverage.branches.covered_count,
            coverage.branches.count
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
                padding: 4px 12px;
                border-radius: 4px;
            }}
            .file-row {{
                font-style: italic;
                background-color: #fff;
                padding: 4px 12px;
                margin-top: 2px;
                border-radius: 4px;
            }}
            .item-name {{
                flex-grow: 1;
            }}
            .coverage-stats {{
                margin-left: 10px;
                width: 100px;
                max-width: 100px;
                min-width: 100px;
            }}
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
