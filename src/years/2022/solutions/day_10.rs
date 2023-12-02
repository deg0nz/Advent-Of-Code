use crate::util::{self, Day};
use color_eyre::eyre::Result;

pub struct Day10 {
    input: Vec<String>,
}

impl Day10 {
    pub fn new() -> Result<Day10> {
        let input = util::read_input(2022, 10, true)?
            .lines()
            .map(|l| l.to_string())
            .collect();

        Ok(Self { input })
    }

    fn print_display(display: &Vec<Vec<char>>) {
        display.iter().for_each(|row| {
            row.iter().for_each(|pixel| {
                print!("{}", pixel);
            });
            println!("");
        });
    }

    fn set_sprite(x: &i32, sprite: &mut Vec<i32>) {
        for (pos, val) in (*x - 1..=*x + 1).enumerate() {
            sprite[pos] = val;
        }
    }
}

impl Day for Day10 {
    fn a(&self) -> Result<String> {
        let mut signal_strengths: Vec<i32> = Vec::new();

        let mut addx_active = false;
        let mut input_iter = self.input.iter().enumerate().peekable();
        let mut cycle = 0;
        let mut x = 1;
        let mut instruction_hold: Option<String> = None;

        loop {
            if let Some(instruction) = input_iter.peek() {
                match cycle {
                    20 | 60 | 100 | 140 | 180 | 220 => signal_strengths.push(cycle * x),
                    _ => (),
                }

                // let line = instruction.0;
                // dbg!(&cycle, &x, &line + 1, &instruction.1);
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
        let mut display: Vec<Vec<char>> = Vec::new();
        let mut current_sprite: Vec<i32> = Vec::new();
        let mut current_row: usize = 0;
        let mut current_row_pos: i32;

        let mut addx_active = false;
        let mut input_iter = self.input.iter().enumerate().peekable();
        let mut cycle = 0;
        let mut x = 1;
        let mut instruction_hold: Option<String> = None;

        display.push(Vec::new());

        for i in 0..=2 {
            current_sprite.push(i);
        }

        loop {
            if cycle == 45 {
                break;
            }

            Day10::set_sprite(&x, &mut current_sprite);
            current_row_pos = (cycle % 40) - 1;

            if cycle > 0 {
                if current_sprite.contains(&current_row_pos) {
                    display[current_row].push('#');
                } else {
                    display[current_row].push('.');
                }

                dbg!(
                    &x,
                    &cycle,
                    &current_row_pos,
                    &display[current_row].last(),
                    &current_sprite
                );
            }

            match cycle {
                40 | 80 | 120 | 160 | 200 | 240 => {
                    display.push(Vec::new());
                    current_row += 1;
                }
                _ => (),
            }

            if let Some(instruction) = input_iter.peek() {
                // let line = instruction.0;
                // dbg!(&cycle, &x, &line + 1, &instruction.1);
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

        Day10::print_display(&display);

        Ok("foo".to_string())
    }

    fn get_title(&self) -> &str {
        "--- Day 10: Cathode-Ray Tube ---"
    }
}
