use core::num;

use crate::util::Day;
use color_eyre::eyre::Result;

pub struct Day04 {
    input: String,
}

impl Day04 {
    pub fn new() -> Result<Day04> {
        let input = Day04::get_input(2023, 4, true)?;

        Ok(Self { input })
    }

    fn get_card_points(line: &str) -> (usize, u32) {
        let mut card_points = 0;

        let mut line_split = line.split(':');
        let card_num = line_split
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let numbers = line_split.last().unwrap();
        let numbers_split = numbers.split('|').collect::<Vec<&str>>();
        let winning_numbers = numbers_split
            .get(0)
            .unwrap()
            .split_whitespace()
            .into_iter()
            .map(|num| num.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let have_numbers = numbers_split
            .get(1)
            .unwrap()
            .split_whitespace()
            .into_iter()
            .map(|num| num.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        have_numbers.iter().for_each(|have_number| {
            if winning_numbers.contains(have_number) {
                if card_points == 0 {
                    card_points += 1;
                } else {
                    card_points *= 2;
                }
            }
        });

        (card_num, card_points)
    }

    // Why did I knew, this will take too long...? :D
    fn collect_cards_recursive<'a>(
        cards_collector: &mut Vec<&'a str>,
        cards: &'a Vec<&str>,
        current_card: &'a str,
    ) {
        let (card_num, points) = Day04::get_card_points(current_card);

        cards_collector.push(current_card);

        for i in card_num..card_num + points as usize {
            if i < cards.len() - 1 {
                cards_collector.push(cards[i]);
                Day04::collect_cards_recursive(cards_collector, cards, cards[i])
            } else {
                cards_collector.push(cards[cards.len() - 1]);
                return;
            }
        }
    }

    // Meh, this takes too long as well
    fn collect_cards_iterative<'a>(cards_collector: &mut Vec<&'a str>, cards: &'a Vec<&str>) {
        let mut cards_stack: Vec<&str> = Vec::new();

        cards_stack.push(cards[0]);

        while cards_stack.len() > 0 {
            let current_card = cards_stack.pop().unwrap();
            let (card_num, points) = Day04::get_card_points(current_card);

            if card_num < cards.len() {
                cards_collector.push(current_card);
            }

            for i in card_num..card_num + points as usize {
                if i < cards.len() {
                    cards_collector.push(cards[i]);
                    cards_stack.push(cards[i]);
                }
            }
        }
    }
}

impl Day for Day04 {
    fn a(&self) -> Result<String> {
        let mut card_points_sum = 0;

        self.input.lines().for_each(|line| {
            let (_, card_points) = Day04::get_card_points(line);
            card_points_sum += card_points;
        });

        Ok(card_points_sum.to_string())
    }

    fn b(&self) -> Result<String> {
        let cards = self.input.lines().collect::<Vec<&str>>();
        let mut cards_collector: Vec<&str> = Vec::new();
        let mut cards_count = vec![1u128; cards.len()];
        cards_count[0] = 0;

        // Day04::collect_cards_recursive(&mut cards_collector, &cards, cards[0]);
        // Day04::collect_cards_iterative(&mut cards_collector, &cards);

        // dbg!(&cards_collector);

        let mut sum = 0;

        for card in cards.iter() {
            let (card_num, points) = Day04::get_card_points(card);

            sum += cards_count[card_num - 1];

            for j in card_num + 1..card_num + 1 + points as usize {
                if j < cards.len() - 1 {
                    cards_count[j] += cards_count[card_num];
                }
            }
        }

        // cards_collector.iter().for_each(|card| {
        //     let (card_num, _points) = Day04::get_card_points(card);

        //     cards_count[card_num] += 1;
        // });

        dbg!(&cards_count);

        Ok(cards_count.iter().sum::<u128>().to_string())
    }

    fn get_title(&self) -> &str {
        "--- Day 4: FooBar ---"
    }
}
