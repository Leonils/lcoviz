#[derive(Debug, PartialEq)]
struct ReportTree { }

impl ReportTree {
    pub fn from_original_report(report: lcov::report::Report) -> Self {
        ReportTree { } 
    }
}

#[cfg(test)]
mod test {
    use super::ReportTree;

    #[test]
    fn when_building_tree_with_an_empty_report_it_should_get_an_empty_report() {
        let original_report = lcov::report::Report::new();
        let report_tree = ReportTree::from_original_report(original_report);
        assert_eq!(ReportTree {}, report_tree);
    }
}
