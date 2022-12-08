use crate::util::{self, Day};
use color_eyre::eyre::Result;

pub struct Day08 {
    grid: Vec<Vec<u32>>,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Day08 {
    pub fn new() -> Result<Day08> {
        let input = util::read_input(2022, 8, false)?;
        let mut grid: Vec<Vec<u32>> = Vec::new();

        input.lines().enumerate().for_each(|(i, line)| {
            grid.push(Vec::new());
            line.chars().for_each(|c| {
                grid[i].push(u32::from_str_radix(c.to_string().as_str(), 10).unwrap());
            })
        });

        Ok(Self { grid })
    }

    fn check_direction(&self, pos: (usize, usize), direction: Direction) -> bool {
        let height = self.grid[pos.0][pos.1];
        let start: usize;
        let end: usize;
        let vertical: bool;
        let row = pos.0;
        let col = pos.1;

        match direction {
            Direction::Left => {
                start = 0;
                end = col - 1;
                vertical = false;
            }
            Direction::Right => {
                start = col + 1;
                end = self.grid.len() - 1;
                vertical = false;
            }
            Direction::Up => {
                start = 0;
                end = row - 1;
                vertical = true;
            }
            Direction::Down => {
                start = row + 1;
                end = self.grid.len() - 1;
                vertical = true;
            }
        }

        for i in start..=end {
            let current_height: u32;

            if vertical {
                current_height = self.grid[i][col];
            } else {
                current_height = self.grid[row][i];
            }

            if current_height >= height {
                return false;
            }
        }

        true
    }

    fn tree_is_visible(&self, pos: (usize, usize)) -> bool {
        self.check_direction(pos, Direction::Left)
            || self.check_direction(pos, Direction::Right)
            || self.check_direction(pos, Direction::Up)
            || self.check_direction(pos, Direction::Down)
    }
}

impl Day for Day08 {
    fn a(&self) -> Result<String> {
        let height = self.grid.len();
        let width = self.grid.first().unwrap().len();
        let mut num_visible_trees = (height * 2 + width * 2) - 4;

        for i in 1..height - 1 {
            for j in 1..width - 1 {
                if self.tree_is_visible((i, j)) {
                    num_visible_trees += 1;
                }
            }
        }

        Ok(num_visible_trees.to_string())
    }

    fn b(&self) -> Result<String> {
        todo!()
    }

    fn get_title(&self) -> &str {
        "--- Day 8: Treetop Tree House ---"
    }
}
