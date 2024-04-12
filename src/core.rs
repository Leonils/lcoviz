pub trait AggregatedCoverageCounters {
    fn get_count(&self) -> u32;
    fn get_covered_count(&self) -> u32;
}

pub trait AggregatedCoverage {
    fn get_lines(&self) -> Option<impl AggregatedCoverageCounters>;
    fn get_functions(&self) -> Option<impl AggregatedCoverageCounters>;
    fn get_branches(&self) -> Option<impl AggregatedCoverageCounters>;
}

pub trait TestedFile {
    fn get_file_path(&self) -> String;
    fn get_aggregated_coverage(&self) -> impl AggregatedCoverage;
}

pub trait TestedContainer {
    fn get_aggregated_coverage(&self) -> impl AggregatedCoverage;
    fn get_container_children(&self) -> Vec<impl TestedContainer>;
    fn get_code_file_children(&self) -> Vec<impl TestedFile>;
}

pub trait Renderer {
    fn render_coverage_summary(&self, root: impl TestedContainer) -> &str;
    fn render_file_coverage_details(&self, file: impl TestedFile) -> &str;
}
