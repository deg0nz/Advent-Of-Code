use crate::util::{self, Day};
use color_eyre::eyre::Result;
use regex::Regex;

#[derive(Debug)]
struct Movement {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}

pub struct Day05 {
    data: String,
}

impl Day05 {
    pub fn new() -> Result<Day05> {
        let input = util::read_input(2022, 5, false)?;

        Ok(Self { data: input })
    }

    fn parse_stacks(&self) -> Result<Vec<Vec<char>>> {
        let input = self.data.lines().collect::<Vec<&str>>();
        let matrix_input = &input[0..=7];
        let mut matrix: Vec<Vec<char>> = Vec::new();

        let column_indices = input
            .get(8)
            .unwrap()
            .chars()
            .into_iter()
            .enumerate()
            .filter_map(|(i, c)| {
                if c.is_digit(10) {
                    return Some(i);
                }

                None
            })
            .collect::<Vec<usize>>();

        matrix_input.iter().rev().for_each(|row| {
            let row_chars = row.chars().collect::<Vec<char>>();
            for (matrix_col_index, row_chars_index) in column_indices.iter().enumerate() {
                let current_pos = row_chars[*row_chars_index].clone();
                if current_pos.is_alphabetic() {
                    if let None = matrix.get(matrix_col_index) {
                        matrix.push(Vec::new());
                    }

                    matrix[matrix_col_index].push(current_pos)
                }
            }
        });

        Ok(matrix)
    }

    fn parse_movement(line: &str) -> Result<Movement> {
        let re = Regex::new(r"\d+")?;
        let matches = re
            .find_iter(line)
            .map(|m| usize::from_str_radix(m.as_str(), 10).unwrap())
            .collect::<Vec<usize>>();

        let mv = Movement {
            amount: matches[0] - 1,
            from: matches[1] - 1,
            to: matches[2] - 1,
        };

        Ok(mv)
    }

    fn get_movements(&self) -> Vec<Movement> {
        self
            .data
            .lines()
            .skip(10)
            .filter_map(|line| Some(Day05::parse_movement(line).unwrap()))
            .collect::<Vec<Movement>>()
    }
}

impl Day for Day05 {
    fn a(&self) -> Result<String> {
        let mut matrix = self.parse_stacks()?;
        let movements = self.get_movements();

        movements.iter().for_each(|mv| {
            let col_len = matrix[mv.from].len();
            let removed = matrix[mv.from]
                .drain((col_len - 1 - mv.amount)..)
                .collect::<Vec<char>>();

            removed.iter().rev().for_each(|c| {
                matrix[mv.to].push(*c);
            });
        });

        let top_boxes: String = matrix.iter().map(|col| *col.last().unwrap()).collect();

        Ok(top_boxes)
    }

    fn b(&self) -> Result<String> {
        let mut matrix = self.parse_stacks()?;
        let movements = self.get_movements();

        movements.iter().for_each(|mv| {
            let col_len = matrix[mv.from].len();
            let removed = matrix[mv.from]
                .drain((col_len - 1 - mv.amount)..)
                .collect::<Vec<char>>();

            removed.iter().for_each(|c| {
                matrix[mv.to].push(*c);
            });
        });

        let top_boxes: String = matrix.iter().map(|col| *col.last().unwrap()).collect();

        Ok(top_boxes)
    }

    fn get_title(&self) -> &str {
        "--- Day 5: Supply Stacks ---"
    }
}
