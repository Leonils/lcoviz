#[macro_export]
macro_rules! assert_html_eq {
    ($actual: expr, $($expected: expr),*) => {
        let actual = $actual;
        let expected = concat!($($expected),*);
        if (actual == expected) {
            return;
        }

        let first_diff = actual.chars().zip(expected.chars()).position(|(a, b)| a != b);
        if let Some(pos) = first_diff {
            let pos_5_before = pos.saturating_sub(5);
            let offset = pos - pos_5_before;
            let before_actual = &actual[..pos_5_before];
            let after_actual = &actual[pos_5_before..pos+10];
            let after_expected = &expected[pos_5_before..pos+10];
            panic!(
                r"----------------
Strings differ:

# Common: 
{}

# Actual/Expected
-> {}
-> {}
   {}^
----------------
",
                before_actual, after_actual, after_expected, " ".repeat(offset)
            );
        }
        else {
            panic!("Strings differ: \nActual:   {}\nExpected: {}", actual, expected);
        }
    };
}
