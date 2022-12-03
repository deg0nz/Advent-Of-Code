use super::super::super::util::InputReader;
use crate::util::Day;
use color_eyre::eyre::Result;

pub struct Day01 {
    data: Vec<u32>,
}

impl Day01 {
    pub fn new() -> Result<Day01> {
        let util = InputReader::new(2022, "01".to_string(), false);
        let input = util.read()?;

        let mut elf_supplies: Vec<u32> = Vec::new();
        let mut current_supply_counter = 0;

        input.lines().for_each(|line| {
            if !line.is_empty() {
                let current = line.parse::<u32>().unwrap();
                current_supply_counter += current;
            } else {
                elf_supplies.push(current_supply_counter);
                current_supply_counter = 0;
            }
        });

        Ok(Self { data: elf_supplies })
    }
}

impl Day for Day01 {
    fn a(&self) -> Result<String> {
        let elf_with_most_dupplies = self.data.iter().max().unwrap();
        Ok(elf_with_most_dupplies.to_string())
    }

    fn b(&self) -> Result<String> {
        let mut data_sorted = self.data.clone();
        data_sorted.sort();
        let supply_sum_top_3: u32 = data_sorted.split_off(data_sorted.len() - 3).iter().sum();

        Ok(supply_sum_top_3.to_string())
    }

    fn print_title(&self) {
        println!("--- Day 1: Calorie Counting ---");
    }
}