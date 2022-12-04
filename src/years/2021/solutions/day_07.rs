use color_eyre::eyre::Result;
use crate::util::Day;

pub struct Day07 {
    positions: Vec<i32>,
}

impl Day07 {
    pub fn new() -> Result<Day07> {
        let data = Day07::get_input(2021, 7, false)?;

        let positions = data
            .split(",")
            .into_iter()
            .map(|val| val.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        Ok(Self { positions })
    }

    fn median(numbers: &mut Vec<i32>) -> i32 {
        numbers.sort();
        let mid = numbers.len() / 2;
        numbers[mid]
    }
}

impl Day for Day07 {
    fn a(&self) -> Result<String> {
        let mut positions = self.positions.clone();
        let median = Day07::median(&mut positions);

        let mut fuel_cost: i32 = 0;

        self.positions.iter().for_each(|pos| {
            fuel_cost += (*pos - median).abs()
        });

        let print = format!("Median: {} | Fuel cost: {}", median, fuel_cost);

        Ok(print)
    }

    fn b(&self) -> Result<String> {
        let mut min_fuel = std::i32::MAX;

        self.positions.iter().enumerate().for_each(|(i, _pos_outer)| {
            let mut current_fuel = 0;

            self.positions.iter().for_each(|pos_inner| {
                let diff = (i as i32 - pos_inner).abs();
                let fuel: i32 = (diff * (diff + 1) / 2) as i32;

                current_fuel += fuel;
            });

            if current_fuel < min_fuel {
                min_fuel = current_fuel;
            }
        });

        let print = format!("Fuel cost: {}", min_fuel);

        Ok(print)
    }

    fn get_title(&self) -> &str {
        "--- Day 7: The Treachery of Whales ---"
    }
}