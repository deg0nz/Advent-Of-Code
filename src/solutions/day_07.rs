use color_eyre::eyre::Result;

use crate::util::Day;

use super::super::util::Util;

pub struct Day07 {
    positions: Vec<u32>,
}

impl Day07 {
    pub fn new() -> Result<Day07> {
        println!("");
        println!("========= Day 07: The Treachery of Whales =========");

        let util = Util::new();
        let data = util.read_input("day_07.txt")?;

        let positions = data
            .split(",")
            .into_iter()
            .map(|val| val.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        Ok(Self { positions })
    }

    fn median(numbers: &mut Vec<u32>) -> u32 {
        numbers.sort();
        let mid = numbers.len() / 2;
        numbers[mid]
    }
}

impl Day for Day07 {
    fn a(&self) -> Result<()> {
        let mut positions = self.positions.clone();
        let median = Day07::median(&mut positions);

        let mut fuel_cost: i32 = 0;

        self.positions.iter().for_each(|pos| {
            fuel_cost += (*pos as i32 - median as i32).abs()
        });

        println!("[A] Median: {} | Fuel cost: {}", median, fuel_cost);

        Ok(())
    }

    fn b(&self) -> Result<()> {
        todo!()
    }
}