use std::collections::HashMap;

use crate::util::{self, Day};
use color_eyre::eyre::Result;

pub struct Day08 {
    input: String,
}

impl Day08 {
    pub fn new() -> Result<Day08> {
        let input = util::read_input(2021, 8, false)?;

        Ok(Self { input })
    }

    fn get_left(line: &str) -> Vec<&str> {
        let output_side = line.split("|").next().unwrap();
        output_side.split_whitespace().collect::<Vec<&str>>()
    }

    fn get_right(line: &str) -> Vec<&str> {
        let output_side = line.split("|").last().unwrap();
        output_side.split_whitespace().collect::<Vec<&str>>()
    }

    fn sort_str(sig: &String) -> String {
        let mut sig_mut: Vec<char> = String::from(sig).chars().collect();
        sig_mut.sort_by(|a, b| a.cmp(b));
        let sig_string = String::from_iter(sig_mut);

        sig_string
    }

    fn identify_signal(
        &self,
        sig: &str,
        signals_pattern: &HashMap<usize, String>,
    ) -> Option<String> {
        let mut sig_mut: Vec<char> = String::from(sig).chars().collect();
        sig_mut.sort_by(|a, b| a.cmp(b));

        let sig_string = String::from_iter(sig_mut);

        let mut identified = None;

        signals_pattern.iter().for_each(|(index, pattern)| {
            let sorted_pattern = Day08::sort_str(pattern);
            if sorted_pattern.eq(&sig_string) {
                identified = Some(index.to_string())
            }
        });

        identified
    }

    fn get_pattern_map(line: Vec<&str>) -> HashMap<usize, String> {
        let mut pattern_map: HashMap<usize, String> = HashMap::new();

        // Find out unique numbers
        let uniques_filtered = line
            .iter()
            .filter(|&&pattern| {
                match pattern.len() {
                    2 => {
                        // 1
                        pattern_map.insert(1, pattern.to_string());
                        return false;
                    }
                    3 => {
                        // 7
                        pattern_map.insert(7, pattern.to_string());
                        return false;
                    }
                    4 => {
                        // 4
                        pattern_map.insert(4, pattern.to_string());
                        return false;
                    }
                    7 => {
                        // 8
                        pattern_map.insert(8, pattern.to_string());
                        return false;
                    }
                    _ => return true,
                }
            })
            .collect::<Vec<&&str>>();

        // Filter signals with 6 segments
        let sixers = uniques_filtered
            .iter()
            .filter(|&&&pattern| {
                if pattern.len() == 6 {
                    // 9 contains 4
                    if Day08::a_contains_b(pattern, pattern_map.get(&4).unwrap()) {
                        pattern_map.insert(9, pattern.to_string());

                        return false;
                    }

                    // 0 contains 1 but not 4
                    if Day08::a_contains_b(pattern, pattern_map.get(&1).unwrap())
                        && !Day08::a_contains_b(pattern, pattern_map.get(&4).unwrap())
                    {
                        pattern_map.insert(0, pattern.to_string());
                        return false;
                    }

                    return true;
                } else {
                    return false;
                }
            })
            .collect::<Vec<&&&str>>();

        // 6 is left
        pattern_map.insert(6, sixers.last().unwrap().to_string());

        // Filter signals wih 5 segments
        let fivers = uniques_filtered
            .iter()
            .filter(|&&&pattern| {
                if pattern.len() == 5 {
                    // 3 has 1
                    if Day08::a_contains_b(pattern, pattern_map.get(&1).unwrap()) {
                        pattern_map.insert(3, pattern.to_string());

                        return false;
                    }

                    // 5 is in 6
                    if Day08::a_contains_b(pattern_map.get(&6).unwrap(), pattern) {
                        pattern_map.insert(5, pattern.to_string());
                        return false;
                    }

                    return true;
                } else {
                    return false;
                }
            })
            .collect::<Vec<&&&str>>();

        // 2 is left
        pattern_map.insert(2, fivers.last().unwrap().to_string());

        pattern_map
    }

    fn a_contains_b(a: &str, b: &str) -> bool {
        b.chars()
            .into_iter()
            .filter(|&current| a.contains(current))
            .collect::<Vec<char>>()
            .len()
            == b.len()
    }
}

impl Day for Day08 {
    fn a(&self) -> Result<String> {
        let mut num_unique_outputs = 0;

        self.input.lines().for_each(|line| {
            let output = Day08::get_right(line);
            output
                .iter()
                .for_each(|output_value| match output_value.len() {
                    2 | 4 | 3 | 7 => num_unique_outputs += 1,
                    _ => (),
                });
        });

        Ok(num_unique_outputs.to_string())
    }

    fn b(&self) -> Result<String> {
        let mut decoded_output = 0;

        self.input.lines().for_each(|line: &str| {
            let left = Day08::get_left(line);
            let pattern = Day08::get_pattern_map(left);

            let right = Day08::get_right(line);

            let mut decoded_line: Vec<String> = Vec::new();
            right.iter().for_each(|signal| {
                if let Some(identified_signal) = self.identify_signal(*signal, &pattern) {
                    decoded_line.push(identified_signal);
                }
            });

            let decoded_int: i32 = decoded_line.join("").parse().unwrap();
            decoded_output += decoded_int;
        });

        Ok(decoded_output.to_string())
    }

    fn get_title(&self) -> &str {
        "--- Day 8: Seven Segment Search ---"
    }
}
