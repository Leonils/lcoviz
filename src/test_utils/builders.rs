use std::path::PathBuf;

use lcov::report::{
    section::{
        branch::{Branches, Key as BranchKey, Value as BranchValue},
        function::{Functions, Key as FunctionKey, Value as FunctionValue},
        line::{Key as LineKey, Lines, Value as LineValue},
        Key as SectionKey, Value as SectionValue,
    },
    Report,
};

pub trait FromStr {
    fn from_str(text: &str) -> Self;
}
pub trait FromLineNumber {
    fn from_line_number(line_number: u32) -> Self;
}

pub trait FromCount {
    fn from_count(count: u64) -> Self;
}

pub trait InsertLine {
    fn insert_line(&mut self, line_number: u32, count: u64) -> &mut Self;
}

pub trait InsertFunction {
    fn insert_function(&mut self, name: &str, count: u64) -> &mut Self;
}

pub trait InsertBranch {
    fn insert_branch(&mut self, line_number: u32, count: u64) -> &mut Self;
}

pub trait InsertSection {
    fn insert_section(self, key: SectionKey, value: SectionValue) -> Self;
    fn insert_empty_section(self, key: SectionKey) -> Self;
}

impl FromStr for SectionKey {
    fn from_str(text: &str) -> Self {
        SectionKey {
            source_file: PathBuf::from(text),
            test_name: String::from(""),
        }
    }
}

impl FromLineNumber for LineKey {
    fn from_line_number(line_number: u32) -> Self {
        LineKey { line: line_number }
    }
}

impl FromCount for LineValue {
    fn from_count(count: u64) -> Self {
        LineValue {
            count,
            ..Default::default()
        }
    }
}

impl InsertLine for Lines {
    fn insert_line(&mut self, line_number: u32, count: u64) -> &mut Self {
        self.insert(
            LineKey::from_line_number(line_number),
            LineValue::from_count(count),
        );
        self
    }
}

impl InsertFunction for Functions {
    fn insert_function(&mut self, name: &str, count: u64) -> &mut Self {
        self.insert(
            FunctionKey {
                name: String::from(name),
            },
            FunctionValue {
                count,
                ..Default::default()
            },
        );
        self
    }
}

impl InsertBranch for Branches {
    fn insert_branch(&mut self, line: u32, count: u64) -> &mut Self {
        self.insert(
            BranchKey {
                line,
                block: 0,
                branch: 0,
            },
            BranchValue {
                taken: Some(count),
                ..Default::default()
            },
        );
        self
    }
}

impl InsertSection for Report {
    fn insert_section(mut self, key: SectionKey, value: SectionValue) -> Self {
        self.sections.insert(key, value);
        self
    }

    fn insert_empty_section(self, key: SectionKey) -> Self {
        self.insert_section(key, SectionValue::default())
    }
}

pub fn generate_3_lines_2_covered_section() -> SectionValue {
    let mut section_value = SectionValue::default();
    section_value
        .lines
        .insert_line(1, 3)
        .insert_line(2, 0)
        .insert_line(3, 1);
    section_value
}

pub fn generate_2_lines_1_covered_section() -> SectionValue {
    let mut section_value = SectionValue::default();
    section_value.lines.insert_line(1, 3).insert_line(2, 0);
    section_value
}
