use crate::util::{self, Day};
use color_eyre::eyre::Result;

pub struct Day06 {
    data: String,
}

impl Day06 {
    pub fn new() -> Result<Day06> {
        let input = util::read_input(2022, 6, false)?;

        Ok(Self { data: input })
    }

    fn is_marker(slice: &str) -> bool { 
        let duplicates = slice
            .chars() // We iterate the slice's chars Vec and filter:
            .filter(|c| slice.matches(*c).count() > 1) // If the current char has more than 1 ocurrence, it stays
            .count(); // If the slice has only unique chars, there should be no elements left

        duplicates == 0
    }

    fn get_marker_pos(&self, distinct_chars_num: usize) -> usize {
        let input = self.data.clone();
        let marker_pos = input
            .chars()
            .enumerate()
            .position(|(i, _c)| {
                if i > input.len() - distinct_chars_num - 1 {
                    return false;
                }

                let current_slice = &input[i..i + distinct_chars_num];
                Day06::is_marker(current_slice)
            })
            .unwrap();

        marker_pos + distinct_chars_num
    }
}

impl Day for Day06 {
    fn a(&self) -> Result<String> {
        let marker_pos = self.get_marker_pos(4);
        Ok(format!("First message marker with 4 distinct chars: {}", marker_pos.to_string()))
    }

    fn b(&self) -> Result<String> {
        let marker_pos = self.get_marker_pos(14);
        Ok(format!("First message marker with 14 distinct chars: {}", marker_pos.to_string()))
    }

    fn get_title(&self) -> &str {
        "--- Day 6: Tuning Trouble ---"
    }
}
