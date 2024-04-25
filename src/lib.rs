pub mod file_provider;

pub mod html {
    pub mod colors;
    pub mod components;
}

pub mod aggregation {
    mod aggregated;
    pub mod input;
    pub mod multi_report;
    pub mod tested_file;
    mod tested_module;
    pub mod tested_root;
    pub mod with_path;

    #[cfg(test)]
    pub mod fixtures;
}

pub mod core;

pub mod adapters {
    pub mod renderers {
        mod file_icon;
        pub mod html_light_renderer;

        pub mod components {
            pub mod chip;
            pub mod code_line;
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
}

pub mod cli {
    pub mod parser;
}

#[cfg(test)]
pub mod test_utils {
    pub mod builders;
    pub mod macros;
}
