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

    fn get_left_right_values(&self) -> (Vec<i64>, Vec<i64>) {
        let mut left: Vec<i64> = vec![];
        let mut right: Vec<i64> = vec![];

        self.input.lines().for_each(|line| {
            let values: Vec<&str> = line.split("   ").collect();
            left.push(values[0].parse().unwrap());
            right.push(values[1].parse().unwrap());
        });

        (left, right)
    }
}

impl Day for Day01 {
    fn a(&self) -> Result<String> {
        let (mut left, mut right) = self.get_left_right_values();
        let mut distance_sum = 0;

        left.sort();
        right.sort();

        left.iter().enumerate().for_each(|(i, _val)| {
            let distance = (left[i] - right[i]).abs();
            distance_sum += distance;
        });

        Ok(distance_sum.to_string())
    }

    fn b(&self) -> Result<String> {
        let (left, right) = self.get_left_right_values();
        let mut similarity_score_sum = 0;

        left.iter().for_each(|left_val| {
            let num_val = right
                .iter()
                .filter(|right_val| left_val == *right_val)
                .count() as i64;

            similarity_score_sum += left_val * num_val
        });

        Ok(similarity_score_sum.to_string())
    }

    fn get_title(&self) -> &str {
        "--- Day 1: Historian Hysteria ---"
    }
}
