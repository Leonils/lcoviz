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
    mod aggregated;
    mod tested_file;
    mod tested_module;
    pub mod tested_root;
    mod with_path;

    #[cfg(test)]
    pub mod fixtures;
}

mod views {
    mod index;
}

mod core;

#[cfg(test)]
pub mod test_utils {
    pub mod builders;
    pub mod mocks;
}
