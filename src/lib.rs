mod file_provider;

mod html {
    pub mod colors;
    pub mod components;
}

mod aggregation {
    mod aggregated;
    pub mod multi_report;
    pub mod tested_file;
    mod tested_module;
    pub mod tested_root;
    mod with_path;

    #[cfg(test)]
    pub mod fixtures;
}

mod core;
pub mod operations;

pub mod adapters {
    pub mod renderers {
        pub mod html_light_renderer;

        mod components {
            pub mod chip;
            pub mod code_line;
            pub mod file_icon;
            pub mod function;
            pub mod gauges;
            pub mod navigation;
        }

        #[cfg(test)]
        pub mod mock_renderer;
    }

    pub mod exporters {
        pub mod mpa;
        pub mod mpa_links;
    }

    pub mod cli {
        pub mod cli_output;
        pub mod parser;
    }
}

mod input {
    pub mod aggregator_input;
    pub mod config;
}

#[cfg(test)]
mod test_utils {
    pub mod builders;
    mod macros;
}
