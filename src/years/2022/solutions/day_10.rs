use crate::util::{self, Day};
use color_eyre::eyre::Result;

pub struct Day10 {
    input: Vec<String>,
}

impl Day10 {
    pub fn new() -> Result<Day10> {
        let input = util::read_input(2022, 10, false)?
            .lines()
            .map(|l| l.to_string())
            .collect();

        Ok(Self { input })
    }
}

impl Day for Day10 {
    fn a(&self) -> Result<String> {
        let mut cycle = 0;
        let mut x = 1;
        let mut x_hold = 0;
        let mut addx_active = false;

        let mut instruction_hold: Option<String> = None;

        let mut input_iter = self.input.iter().enumerate().peekable();

        let mut signal_strengths: Vec<i32> = Vec::new();

        loop {
            if let Some(instruction) = input_iter.peek() {
                match cycle {
                    20 | 60 | 100 | 140 | 180 | 220 => signal_strengths.push(cycle * x),
                    _ => (),
                }

                let line = instruction.0;
                dbg!(&cycle, &x, &line + 1, &instruction.1);
                if instruction.1.as_str() == "noop" {
                    if let Some(i) = &instruction_hold {
                        x += i32::from_str_radix(i.split_once(" ").unwrap().1, 10)?;
                        instruction_hold = None;
                    }
                    input_iter.next();
                } else {
                    if addx_active {
                        addx_active = false;
                    } else {
                        if let Some(instruction) = input_iter.next() {
                            // x += x_hold;
                            if let Some(i) = &instruction_hold {
                                x += i32::from_str_radix(i.split_once(" ").unwrap().1, 10)?;
                            }

                            instruction_hold = Some(instruction.1.clone());

                            addx_active = true;
                        }
                    }
                }
            } else {
                break;
            }

            cycle += 1;
        }

        dbg!(&signal_strengths);

        let signal_strengths_sum: i32 = signal_strengths.iter().sum();

        Ok(signal_strengths_sum.to_string())
    }

    fn b(&self) -> Result<String> {
        todo!()
    }

    fn get_title(&self) -> &str {
        "--- Day 10: Cathode-Ray Tube ---"
    }
}
