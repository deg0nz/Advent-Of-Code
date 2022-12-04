use color_eyre::eyre::Result;
use crate::util::Day;

pub struct Day01 {
    data: Vec<u32>,
}

impl Day01 {
    pub fn new() -> Result<Day01> {
        let input = Day01::get_input(2021, 1, false)?;
        let data = input
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .collect();

        Ok(Self { data })
    }

    fn sum_window(&self, window: &[u32]) -> u32 {
        let mut sum = 0;

        window.iter().for_each(|elem| sum += *elem);

        sum
    }
}

impl Day for Day01 {
    fn a(&self) -> Result<String> {
        // dbg!(&self.data);
        let mut increase_counter: u32 = 0;
        let mut iter = self.data.iter().peekable();

        while let Some(current) = iter.next() {
            if let Some(next) = iter.peek() {
                if *current < **next {
                    increase_counter += 1;
                }
            }
        }

        Ok(increase_counter.to_string())
    }

    fn b(&self) -> Result<String> {
        let mut increase_counter: u32 = 0;
        let mut windows = self.data.windows(3).peekable();

        while let Some(current) = windows.next() {
            if let Some(next) = windows.peek() {
                let sum_current = self.sum_window(current);
                let sum_next = self.sum_window(next);

                if sum_current < sum_next {
                    increase_counter += 1;
                }
            };
        }

        Ok(increase_counter.to_string())
    }

    fn get_title(&self) -> &str {
        "--- Day 1: Sonar Sweep ---"
    }
}
