use color_eyre::eyre::Result;

use crate::util::Day;

use super::super::util::Util;

pub struct Day01 {
    data: Vec<u32>,
}

impl Day01 {
    pub fn new() -> Result<Day01> {
        let util = Util::new();
        let input = util.read_input::<u32>("day_01.txt");

        let data = input.iter().map(|x| x.to_owned().unwrap()).collect();

        Ok(Self { data })
    }

    fn sum_window(&self, window: &[u32]) -> u32 {
        let mut sum = 0;

        window.iter().for_each(|elem| sum += *elem);

        sum
    }
}

impl Day for Day01 {
    fn a(&self) {
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

        println!("Depth A: {}", increase_counter);
    }

    fn b(&self) {
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

        println!("Depth B: {}", increase_counter);
    }
}
