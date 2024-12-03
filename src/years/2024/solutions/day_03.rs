use crate::util::Day;
use color_eyre::eyre::Result;
use regex::Regex;

pub struct Day03 {
    input: String,
}

impl Day03 {
    pub fn new() -> Result<Day03> {
        let input = Day03::get_input(2024, 3, false)?;

        Ok(Self { input })
    }
}

impl Day for Day03 {
    fn a(&self) -> Result<String> {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let mut result: u32 = 0;

        for (_, [m1, m2]) in re.captures_iter(self.input.as_str()).map(|c| c.extract()) {
            result += m1.parse::<u32>().unwrap() * m2.parse::<u32>().unwrap()
        }

        Ok(result.to_string())
    }

    fn b(&self) -> Result<String> {
        let re = Regex::new(r"(do\(\))|(don't\(\))|(mul\(\d+,\d+\))").unwrap();
        let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        let mut result: u32 = 0;
        let mut mul_enabled = true;

        for (_, [c]) in re.captures_iter(self.input.as_str()).map(|c| c.extract()) {
            if c.contains("do()") {
                mul_enabled = true;
            } else if c.contains("don't()") {
                mul_enabled = false;
            } else if mul_enabled {
                let mul = re_mul.captures(c).unwrap();
                let m1 = mul.get(1).map(|v| v.as_str()).unwrap();
                let m2 = mul.get(2).map(|v| v.as_str()).unwrap();
                result += m1.parse::<u32>().unwrap() * m2.parse::<u32>().unwrap()
            }
        }

        Ok(result.to_string())
    }

    fn get_title(&self) -> &str {
        "--- Day 3: Mull It Over ---"
    }
}
