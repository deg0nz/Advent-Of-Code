use core::num;

use crate::util::{self, Day};
use color_eyre::eyre::Result;

pub struct Day08 {
    input: String
}

impl Day08 {
    pub fn new() -> Result<Day08> {
        let input = util::read_input(2021, 8, false)?;

        Ok(Self { input })
    }

    fn get_output_values_vec(line: &str) -> Vec<&str> {
        let output_side = line.split("|").last().unwrap();
        output_side.split_whitespace().collect::<Vec<&str>>()
    }

    fn identify_signal(sig: &str) -> &str {
        let mut digit: &str = "";

        match sig {
            "acedgfb" => digit = "8",
            "cdfbe" => digit = "5",
            "gcdfa" =>  digit = "2",
            "fbcad" => digit = "3",
            "dab" => digit = "7",
            "cefabd" => digit = "9",
            "cdfgeb" => digit = "6",
            "eafb" => digit = "4",
            "cagedb" => digit = "0",
            "ab" =>  digit = "1",
            _ => ()
        }

        digit
    }
}

impl Day for Day08 {
    fn a(&self) -> Result<String> {
        let mut num_unique_outputs = 0;

        self.input.lines().for_each(|line| {
            let output = Day08::get_output_values_vec(line);
            output.iter().for_each(|output_value| {
                match output_value.len() {
                    2 | 4 | 3 | 7 => num_unique_outputs += 1,
                    _ => ()
                }
            });
        });

        Ok(num_unique_outputs.to_string())
    }

    fn b(&self) -> Result<String> {
        todo!()
    }

    fn get_title(&self) -> &str {
        "--- Day 8: Seven Segment Search ---"
    }
}