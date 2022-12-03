use super::super::super::util::InputReader;
use crate::util::Day;
use color_eyre::eyre::Result;

pub struct Day02 {
    data: String,
}

impl Day02 {
    pub fn new() -> Result<Day02> {
        let util = InputReader::new(2022, "02".to_string(), false);
        let input = util.read()?;

        Ok(Self { data: input })
    }

    fn get_score_for_round(&self, round: &str) -> u32 {
        let mut score: u32 = 0;
        let my_choice = round.split_whitespace().last().unwrap();

        match my_choice {
            "X" => score += 1,
            "Y" => score += 2,
            "Z" => score += 3,
            &_ => unreachable!(),
        };

        match round {
            // Draw
            "A X" | "B Y" | "C Z" => score += 3,
            // Win
            "A Y" | "B Z" | "C X" => score += 6,
            &_ => (),
        }

        score
    }

    fn get_my_choice_and_needed_outcome<'a>(&'a self, round: &'a str) -> (&str, &str) {
        let round_elements = round.split_whitespace();
        let opponents_choice = round_elements.clone().into_iter().nth(0).unwrap();
        let needed_outcome = round_elements.last().unwrap();

        let my_choice: &str;

        match needed_outcome {
            // Loose
            "X" => match opponents_choice {
                "A" => my_choice = "Z",
                "B" => my_choice = "X",
                "C" => my_choice = "Y",
                &_ => unreachable!(),
            },
            // Draw
            "Y" => match opponents_choice {
                "A" => my_choice = "X",
                "B" => my_choice = "Y",
                "C" => my_choice = "Z",
                &_ => unreachable!(),
            },
            // Win
            "Z" => match opponents_choice {
                "A" => my_choice = "Y",
                "B" => my_choice = "Z",
                "C" => my_choice = "X",
                &_ => unreachable!(),
            },
            &_ => unreachable!(),
        }

        (my_choice, needed_outcome)
    }
}

impl Day for Day02 {
    fn a(&self) -> Result<String> {
        let mut total_score = 0;

        self.data
            .lines()
            .for_each(|round| total_score += self.get_score_for_round(round));

        Ok(total_score.to_string())
    }

    fn b(&self) -> Result<String> {
        let mut total_score = 0;

        self.data.lines().for_each(|round| {
            let (my_choice, needed_outcome) = self.get_my_choice_and_needed_outcome(round);
            let played_round = round.replace(needed_outcome, my_choice);
            total_score += self.get_score_for_round(&played_round);
        });

        Ok(total_score.to_string())
    }

    fn print_title(&self) {
        println!("--- Day 2: Rock Paper Scissors ---")
    }
}
