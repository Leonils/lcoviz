#[derive(Default, Debug, PartialEq)]
pub struct AggregatedCoverageCounters {
    pub count: u32,
    pub covered_count: u32,
}
impl AggregatedCoverageCounters {
    pub fn new(count: u32, covered_count: u32) -> Self {
        AggregatedCoverageCounters {
            count,
            covered_count,
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct AggregatedCoverage {
    pub lines: AggregatedCoverageCounters,
    pub functions: AggregatedCoverageCounters,
    pub branches: AggregatedCoverageCounters,
}

pub trait TestedFile {
    fn get_file_path(&self) -> &str;
    fn get_aggregated_coverage(&self) -> &AggregatedCoverage;
}

pub trait TestedContainer {
    fn get_aggregated_coverage(&self) -> &AggregatedCoverage;
    fn get_container_children(&self) -> &Vec<impl TestedContainer>;
    fn get_code_file_children(&self) -> &Vec<impl TestedFile>;
}

pub trait Renderer {
    fn render_coverage_summary(&self, root: impl TestedContainer) -> &str;
    fn render_file_coverage_details(&self, file: impl TestedFile) -> &str;
}
