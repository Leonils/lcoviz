pub mod file_provider;

pub mod html {
    pub mod colors;
    pub mod components;
}

pub mod aggregation {
    mod aggregated;
    pub mod input;
    pub mod multi_report;
    mod tested_file;
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

        #[cfg(test)]
        pub mod mock_renderer;
    }

    pub mod exporters {
        pub mod mpa;
        pub mod mpa_links;
    }
}

#[cfg(test)]
pub mod test_utils {
    pub mod builders;
}
