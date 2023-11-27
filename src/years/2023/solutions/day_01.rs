use crate::util::Day;
use color_eyre::eyre::Result;

pub struct Day01 {
    data: String
}

impl Day01 {
    pub fn new() -> Result<Day01> {
        let _input = Day01::get_input(2022, 1, false)?;

        Ok(Self { data: "foo".to_string() })
    }
}

impl Day for Day01 {
    fn a(&self) -> Result<String> {
        Ok("day01_a".to_string())
    }

    fn b(&self) -> Result<String> {
        Ok("day01_b".to_string())
    }

    fn get_title(&self) -> &str {
        "--- Day 1 ---"
    }
}
