
use crate::util::Day;
use color_eyre::eyre::Result;

pub struct Day01 {
    input: String,
}


impl Day01 {
    pub fn new() -> Result<Day01> {
        let input = Day01::get_input(2024, 1, false)?;

        Ok(Self { input })
    }
}

impl Day for Day01 {
    fn a(&self) -> Result<String> {
        Ok("a".to_string())
    }

    fn b(&self) -> Result<String> {
        Ok("b".to_string())
    }

    fn get_title(&self) -> &str {
        "--- Day 1: FooBar ---"
    }
}
    