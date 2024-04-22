pub mod file_provider;

pub mod html;

pub mod aggregation {
    mod aggregated;
    pub mod input;
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
        mod common;
        pub mod html_light_renderer;

        #[cfg(test)]
        pub mod mock_renderer;
    }

    pub mod exporters {
        pub mod mpa;
    }
}

#[cfg(test)]
pub mod test_utils {
    pub mod builders;
}
