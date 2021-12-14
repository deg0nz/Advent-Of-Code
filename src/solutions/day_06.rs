use color_eyre::eyre::Result;

use crate::util::Day;

use super::super::util::Util;

pub struct Day06 {
    init_state: Vec<u8>,
}

impl Day06 {
    pub fn new() -> Result<Day06> {
        println!("");
        println!("========= Day 06: Lanternfish =========");

        let util = Util::new();
        let data = util.read_input("day_06.txt")?;

        let init_state = data
            .split(",")
            .into_iter()
            .map(|val| val.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        Ok(Self { init_state })
    }

    fn _print_day(fish_list: &Vec<u8>, day: &i32) {
        print!("Day {}: ", day);
        fish_list.iter().for_each(|f| print!("{},", f));
        println!();
    }


}

impl Day for Day06 {
    fn a(&self) -> Result<()> {
        let days = 80;
        let mut fish_list = self.init_state.clone();

        for _day in 0..days {
            let mut newborn = 0;

            for fish in fish_list.iter_mut() {
                if *fish == 0 {
                    newborn += 1;
                    *fish = 6;
                } else {
                    *fish -= 1;
                }
            }

            for _i in 0..newborn {
                fish_list.push(8);
            }

            // Day06::print_day(&fish_list, &day);
        }

        println!("Number of fish after {} days: {}", days, fish_list.len());

        Ok(())
    }

    fn b(&self) -> Result<()> {
        let days = 256;
        let mut age_map: Vec<u64> = Vec::new();

        for i in 0..=8 {
            age_map.push(self.init_state.iter().filter(|timer| **timer == i).collect::<Vec<&u8>>().len() as u64);
        }

        for _day in 1..=days {
            let mut newborn = 0;

            for age in 0..8 {
                if age == 0 {
                    newborn = age_map[0];
                }
                
                if age < age_map.len() - 1 {
                    age_map[age] = age_map[age + 1];
                }
            }

            age_map[6] += newborn;
            age_map[8] = newborn;
        }

        let number_of_fish = age_map.iter().sum::<u64>();

        println!("Number of fish after {} days: {}", days, number_of_fish);

        Ok(())
    }
}
