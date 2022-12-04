use std::ops::Add;
use crate::util::Day;
use color_eyre::eyre::Result;

pub struct Day03 {
    data: String,
}

impl Day03 {
    pub fn new() -> Result<Day03> {
        let input = Day03::get_input(2022, 3, false)?;

        Ok(Self { data: input })
    }

    fn find_common_item_in_rucksack(rucksack_content: &str) -> char {
        let (compartment_a, compartment_b) = rucksack_content.split_at(rucksack_content.len() / 2);

        let common_item = compartment_a
            .chars()
            .into_iter()
            .find(|&c| compartment_b.contains(c))
            .unwrap()
            .to_string()
            .chars()
            .nth(0)
            .unwrap();

        common_item
    }

    fn find_common_item_between_elves(rucksacks: &[&str]) -> char {
        let common_item = rucksacks[0]
            .chars()
            .into_iter()
            .find(|&c| rucksacks[1].contains(c) && rucksacks[2].contains(c))
            .unwrap()
            .to_string()
            .chars()
            .nth(0)
            .unwrap();

        common_item
    }

    fn get_item_priority(item: char) -> Result<u32> {
        let alphabet = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ];

        let mut prio: u32 = alphabet
            .iter()
            .position(|c| c.eq_ignore_ascii_case(&item))
            .unwrap()
            .add(1)
            .try_into()?;

        if !item.is_lowercase() {
            prio += 26;
        }

        Ok(prio)
    }
}

impl Day for Day03 {
    fn a(&self) -> Result<String> {
        let mut total_prio: u32 = 0;

        self.data.lines().for_each(|rucksack_content| {
            let common_item = Day03::find_common_item_in_rucksack(rucksack_content);
            let item_prio = Day03::get_item_priority(common_item).ok().unwrap();
            total_prio += item_prio
        });

        Ok(total_prio.to_string())
    }

    fn b(&self) -> Result<String> {
        let mut total_prio: u32 = 0;
        self.data
            .lines()
            .collect::<Vec<&str>>()
            .chunks(3)
            .for_each(|group| {
                let common_item = Day03::find_common_item_between_elves(group);
                let item_prio = Day03::get_item_priority(common_item).ok().unwrap();
                total_prio += item_prio;
            });

        Ok(total_prio.to_string())
    }

    fn get_title(&self) -> &str {
        "--- Day 3: Rucksack Reorganization ---"
    }
}
