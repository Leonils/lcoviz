pub mod file_provider;

pub mod html {
    pub mod line_to_html;
}

pub mod models {
    pub mod components;
    pub mod file_lines_provider;
    pub mod html_builder;
    pub mod to_html;
}

pub mod styles {
    pub mod light;
}

pub mod aggregation {
    pub mod test_report;
    mod tested_file;
    mod tested_module;
    mod with_path;
}

#[cfg(test)]
pub mod mocks;
