pub fn render_optional_percentage(percentage: Option<f32>) -> String {
    percentage
        .map(|p| format!("{:.2}%", p))
        .unwrap_or("-".to_string())
}

pub fn get_percentage_class(prefix: &str, percentage: &Option<f32>) -> String {
    percentage
        .map(|p| {
            let ten = (p / 10.).round() as u32;
            format!("{}-{}", prefix, ten)
        })
        .unwrap_or(format!("{}-none", prefix))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_optional_percentage_should_return_percentage() {
        assert_eq!(render_optional_percentage(Some(100.)), "100.00%");
    }

    #[test]
    fn render_optional_percentage_with_many_decimals_shall_keep_2() {
        assert_eq!(render_optional_percentage(Some(100.123456)), "100.12%");
    }

    #[test]
    fn render_optional_percentage_should_return_dash() {
        assert_eq!(render_optional_percentage(None), "-");
    }

    #[test]
    fn get_percentage_class_should_return_none() {
        assert_eq!(get_percentage_class("test", &None), "test-none");
    }

    #[test]
    fn get_percentage_class_should_return_the_nearest_ten() {
        assert_eq!(get_percentage_class("test", &Some(1.)), "test-0");
        assert_eq!(get_percentage_class("test", &Some(5.)), "test-1");
        assert_eq!(get_percentage_class("test", &Some(42.)), "test-4");
        assert_eq!(get_percentage_class("test", &Some(95.)), "test-10");
        assert_eq!(get_percentage_class("test", &Some(99.)), "test-10");
        assert_eq!(get_percentage_class("test", &Some(100.)), "test-10");
    }
}
