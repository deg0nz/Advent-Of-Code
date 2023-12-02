use crate::util::Day;
use color_eyre::eyre::Result;

pub struct Day02 {
    input: String,
}

impl Day02 {
    pub fn new() -> Result<Day02> {
        let input = Day02::get_input(2023, 2, false)?;

        Ok(Self { input })
    }

    fn parse_line(line: &str) -> Result<(u32, Vec<Vec<(u32, &str)>>)> {
        let game_num: u32 = line
            .split(':')
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()?;

        let games = line
            .split(':')
            .last()
            .unwrap()
            .split(';')
            .map(|game| {
                let draws = game
                    .split(',')
                    .map(|draw| {
                        let mut set = draw.split_whitespace();
                        let number_cubes: u32 = set.next().unwrap().parse().unwrap();
                        let color = set.last().unwrap();
                        (number_cubes, color)
                    })
                    .collect::<Vec<(u32, &str)>>();
                draws
            })
            .collect::<Vec<Vec<(u32, &str)>>>();

        Ok((game_num, games))
    }
}

impl Day for Day02 {
    fn a(&self) -> Result<String> {
        let num_red = 12;
        let num_green = 13;
        let num_blue = 14;

        let possible_games = self
            .input
            .lines()
            .map(|line| {
                let game = Day02::parse_line(line).unwrap();
                let flat_game = game.1.iter().flatten().collect::<Vec<&(u32, &str)>>();
                //                  r    g    b
                let mut max_vals: (u32, u32, u32) = (0, 0, 0);

                flat_game.iter().for_each(|(num, color)| match color {
                    &"red" => max_vals.0 = max_vals.0.max(*num),
                    &"blue" => max_vals.1 = max_vals.1.max(*num),
                    &"green" => max_vals.2 = max_vals.2.max(*num),
                    _ => return,
                });

                if max_vals.0 <= num_red && max_vals.1 <= num_blue && max_vals.2 <= num_green {
                    return game.0;
                } else {
                    return 0;
                }
            })
            .collect::<Vec<u32>>();

        let possible_games_sum: u32 = possible_games.iter().sum();

        Ok(possible_games_sum.to_string())
    }

    fn b(&self) -> Result<String> {
        let min_vals_pow = self
            .input
            .lines()
            .map(|line| {
                let game = Day02::parse_line(line).unwrap();
                let flat_game = game.1.iter().flatten().collect::<Vec<&(u32, &str)>>();
                //                  r    g    b
                let mut min_vals: (u32, u32, u32) = (0, 0, 0);

                flat_game.iter().for_each(|(num, color)| match color {
                    &"red" => min_vals.0 = min_vals.0.max(*num),
                    &"blue" => min_vals.1 = min_vals.1.max(*num),
                    &"green" => min_vals.2 = min_vals.2.max(*num),
                    _ => return,
                });

                min_vals.0 * min_vals.1 * min_vals.2
            })
            .collect::<Vec<u32>>();

        let min_vals_pow_sum: u32 = min_vals_pow.iter().sum();

        Ok(min_vals_pow_sum.to_string())
    }

    fn get_title(&self) -> &str {
        "--- Day 2: Cube Conundrum ---"
    }
}
