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
        let input = util::read_input(2022, 8, true)?;
        let mut grid: Vec<Vec<u32>> = Vec::new();

        input.lines().enumerate().for_each(|(i, line)| {
            grid.push(Vec::new());
            line.chars().for_each(|c| {
                grid[i].push(u32::from_str_radix(c.to_string().as_str(), 10).unwrap());
            })
        });

        Ok(Self { grid })
    }

    fn check_direction(&self, pos: (usize, usize), direction: Direction) -> (bool, u32) {
        let height = self.grid[pos.0][pos.1];
        let start: usize;
        let end: usize;
        let vertical: bool;
        let backwards: bool;
        let row = pos.0;
        let col = pos.1;
        let mut viewing_distance: u32 = 0;

        match direction {
            Direction::Left => {
                start = col - 1;
                end = 0;
                vertical = false;
                backwards = true;
            }
            Direction::Right => {
                start = col + 1;
                end = self.grid.len() - 1;
                vertical = false;
                backwards = false;
            }
            Direction::Up => {
                start = row - 1;
                end = 0;
                vertical = true;
                backwards = true;
            }
            Direction::Down => {
                start = row + 1;
                end = self.grid.len() - 1;
                vertical = true;
                backwards = false
            }
        }

        dbg!(&pos);

        let mut i = start;

        while start != end {
            let current_height: u32;

            if vertical {
                current_height = self.grid[i][col];
            } else {
                current_height = self.grid[row][i];
            }

            if current_height >= height {
                dbg!(&start, &end, &i);
                // if start + i < end {
                //     viewing_distance += 1;
                // } 

                return (false, viewing_distance);
            }  

            viewing_distance += 1;

            if backwards {
                i -= 1;
            } else if i < end {
                i += 1;
            }
        }

        (true, viewing_distance)
    }

    fn check_tree_visibility_and_determine_scenic_score(&self, pos: (usize, usize)) -> (bool, u32) {
        let left = self.check_direction(pos, Direction::Left);
        let right = self.check_direction(pos, Direction::Right);
        let up = self.check_direction(pos, Direction::Up);
        let down = self.check_direction(pos, Direction::Down);

        dbg!(&left, &right, &up, &down);

        let scenic_score = left.1 * right.1 * up.1 * down.1;

        (left.0 || right.0 || up.0 || down.0, scenic_score)
    }
}

impl Day for Day08 {
    fn a(&self) -> Result<String> {
        let height = self.grid.len();
        let width = self.grid.first().unwrap().len();
        let mut num_visible_trees = (height * 2 + width * 2) - 4;
        let mut highest_scenic_score = 0;

        for i in 1..height - 1 {
            for j in 1..width - 1 {
                let (visible, scenic_score) =
                    self.check_tree_visibility_and_determine_scenic_score((i, j));
                if visible {
                    num_visible_trees += 1;
                }

                if scenic_score >= highest_scenic_score {
                    highest_scenic_score = scenic_score;
                    // dbg!(&i, &j, &scenic_score);
                }
            }
        }

        println!("----------------------------------------------------------------");
        dbg!(self.check_tree_visibility_and_determine_scenic_score((3, 2)));

        Ok(format!(
            "Visible trees: {} | Highest scenic score: {}",
            num_visible_trees, highest_scenic_score
        ))
    }

    fn b(&self) -> Result<String> {
        todo!()
    }

    fn get_title(&self) -> &str {
        "--- Day 8: Treetop Tree House ---"
    }
}
