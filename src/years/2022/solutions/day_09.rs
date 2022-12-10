use std::{borrow::BorrowMut, collections::VecDeque};

use crate::util::{self, Day};
use color_eyre::eyre::Result;

pub struct Day09 {
    input: Vec<(char, usize)>,
}

impl Day09 {
    pub fn new() -> Result<Day09> {
        let input = util::read_input(2022, 9, false)?
            .lines()
            .map(|line| {
                let split = line.split_once(" ").unwrap();
                (
                    split.0.chars().next().unwrap(),
                    usize::from_str_radix(split.1, 10).unwrap(),
                )
            })
            .collect();

        Ok(Self { input })
    }

    fn tail_needs_to_move(head: &(i32, i32), tail: &(i32, i32)) -> bool {
        let dist = ((tail.0 - head.0).abs(), (tail.1 - head.1).abs());
        dist.0 > 1 || dist.1 > 1
    }

    fn advance_head(direction: char, head: &mut (i32, i32)) -> (i32, i32) {
        let last_head = head.clone();

        match direction {
            'R' => head.0 += 1,
            'L' => head.0 -= 1,
            'U' => head.1 += 1,
            'D' => head.1 -= 1,
            _ => unreachable!(),
        }

        last_head
    }

    fn move_tail(head: (i32, i32), tail: &mut (i32, i32)) -> bool {
        // I was just setting the current tail to the last head at first. Of course that didn't work for Part 2.
        // Couldn't wrap my head around it. The head/tail comparison below is from u/compdog
        // https://www.reddit.com/r/adventofcode/comments/zgwhh1/comment/izk1hzt

        if Day09::tail_needs_to_move(&head, tail) {
            if tail.0 < head.0 {
                tail.0 += 1;
            }

            if tail.0 > head.0 {
                tail.0 -= 1;
            }

            if tail.1 < head.1 {
                tail.1 += 1;
            }

            if tail.1 > head.1 {
                tail.1 -= 1;
            }

            true
        } else {
            false
        }
    }

    fn move_rope(rope: &mut Vec<(i32, i32)>) -> bool {
        let mut last_tail_moved = false;

        for i in (0..rope.len()).rev() {
            let head = rope[i];

            if i > 0 {
                let tail = &mut rope[i - 1];
                last_tail_moved = Day09::move_tail(head, tail) && i == 1;
            }
        }

        last_tail_moved
    }
}

impl Day for Day09 {
    fn a(&self) -> Result<String> {
        let mut tail_movements: Vec<(i32, i32)> = Vec::new();
        let mut head = (0, 0);
        let mut tail = (0, 0);

        self.input.iter().for_each(|(direction, count)| {
            for _i in 0..*count {
                let last_head = Day09::advance_head(*direction, &mut head);
                let tail_moved = Day09::move_tail(head, &mut tail);

                if tail_moved {
                    if !tail_movements.contains(&tail) {
                        tail_movements.push(tail.clone());
                    }
                }
            }
        });

        let num_unique_pos_tail = tail_movements.iter().count();

        Ok(num_unique_pos_tail.to_string())
    }

    fn b(&self) -> Result<String> {
        let mut rope: Vec<(i32, i32)> = Vec::new();
        let mut tail_movements: Vec<(i32, i32)> = Vec::new();

        for _i in 0..10 {
            rope.push((0, 0));
        }

        tail_movements.push((15, 11));

        self.input.iter().for_each(|(direction, count)| {
            for _i in 0..*count {
                let head_idx = rope.len() - 1;
                Day09::advance_head(*direction, &mut rope[head_idx]);

                let tail_moved = Day09::move_rope(&mut rope);

                if tail_moved {
                    if !tail_movements.contains(&rope[0]) {
                        tail_movements.push(rope[0].clone());
                    }
                }
            }
        });

        let num_unique_pos_tail = tail_movements.iter().count();

        Ok(num_unique_pos_tail.to_string())
    }

    fn get_title(&self) -> &str {
        "--- Day 9: Rope Bridge ---"
    }
}
