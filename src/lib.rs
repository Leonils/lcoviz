mod file_provider;

mod html {
    pub(crate) mod colors;
    pub(crate) mod components;
}

mod aggregation {
    mod aggregated;
    pub(crate) mod multi_report;
    pub(crate) mod tested_file;
    mod tested_module;
    pub(crate) mod tested_root;
    mod with_path;

    #[cfg(test)]
    pub(crate) mod fixtures;
}

mod core;
pub mod operations;

pub mod adapters {
    pub(crate) mod renderers {
        pub(crate) mod html_light_renderer;
        pub(crate) mod text_single_page_renderer;

        mod components {
            pub(crate) mod chip;
            pub(crate) mod code_line;
            pub(crate) mod file_icon;
            pub(crate) mod function;
            pub(crate) mod gauges;
            pub(crate) mod navigation;
        }

        #[cfg(test)]
        pub(crate) mod mock_renderer;
    }

    pub(crate) mod exporters {
        pub(crate) mod mpa;
        pub(crate) mod mpa_links;
        pub(crate) mod spa;
    }

    pub mod cli {
        pub(crate) mod cli_output;
        pub mod parser;
    }
}

mod input {
    pub(crate) mod aggregator_input;
    pub(crate) mod config;
}

#[cfg(test)]
mod test_utils {
    pub(crate) mod builders;
    mod macros;
}
