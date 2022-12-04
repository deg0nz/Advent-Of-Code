use crate::util::Day;
use color_eyre::eyre::Result;

pub struct Day02 {
    data: String,
}

impl Day02 {
    pub fn new() -> Result<Day02> {
        let data = Day02::get_input(2021, 2, false)?;
        Ok(Self { data })
    }

    fn parse_line_into_values(line: &String) -> Result<(String, u32)> {
        let mut iter = line.split_whitespace();

        let direction = String::from(iter.next().unwrap());
        let value = iter.next().unwrap().parse::<u32>()?;

        Ok((direction, value))
    }
}

impl Day for Day02 {
    fn a(&self) -> Result<String> {
        let mut horizontal: u32 = 0;
        let mut depth: u32 = 0;

        for line in self.data.lines() {
            let values = Day02::parse_line_into_values(&line.to_string())?;

            match values.0.as_str() {
                "forward" => horizontal += values.1,
                "up" => depth -= values.1,
                "down" => depth += values.1,
                _ => unreachable!(),
            }
        }

        Ok((depth * horizontal).to_string())
    }

    fn b(&self) -> Result<String> {
        let mut horizontal: u32 = 0;
        let mut depth: u32 = 0;
        let mut aim: u32 = 0;

        for line in self.data.lines() {
            let values = Day02::parse_line_into_values(&line.to_string())?;

            match values.0.as_str() {
                "forward" => {
                    horizontal += values.1;
                    depth += aim * values.1;
                }
                "up" => aim -= values.1,
                "down" => aim += values.1,
                _ => unreachable!(),
            }
        }

        Ok((depth * horizontal).to_string())
    }

    fn get_title(&self) -> &str {
        "--- Day 2: Dive! ---"
    }
}
