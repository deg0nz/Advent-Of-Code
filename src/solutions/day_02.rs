use color_eyre::eyre::Result;

use crate::util::{Util, Day};

pub struct Day02 {
    data: Vec<String>,
}

impl Day02 {
    pub fn new() -> Result<Day02> {
        let util = Util::new();
        let input = util.read_input::<String>("day_02.txt");

        let data: Vec<String> = input
            .iter()
            .map(|x| x.to_owned().unwrap())
            .collect();

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
    fn a(&self) -> Result<()> {
        let mut horizontal: u32 = 0;
        let mut depth: u32 = 0;

        for line in &self.data {
            let values = Day02::parse_line_into_values(line)?;

            match values.0.as_str() {
                "forward" => horizontal += values.1,
                "up" => depth -= values.1,
                "down" => depth += values.1,
                _ => unreachable!()
            }
        }

        println!("Depth: {}", depth);
        println!("Horizontal: {}", horizontal);
        println!("Multiplied: {}", depth * &horizontal);

        Ok(())
    }

    fn b(&self) -> Result<()> {
        todo!()
    }
}
