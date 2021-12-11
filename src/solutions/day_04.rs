use color_eyre::eyre::Result;

use crate::util::Day;

use super::super::util::Util;

pub struct Day04 {
    draw: Vec<u32>,
    boards: Vec<[[u32; 5]; 5]>,
    boards_marker: Vec<[[bool; 5]; 5]>,
}

impl Day04 {
    pub fn new() -> Result<Day04> {
        println!("");
        println!("========= Day 04: Giant Squid =========");

        let util = Util::new();
        let input = util.read_input("day_04.txt")?;

        let draw = input
            .lines()
            .nth(0)
            .unwrap()
            .split(',')
            .map(|num| num.parse::<u32>().unwrap())
            .collect();

        let mut boards: Vec<[[u32; 5]; 5]> = Vec::new();

        input
            .lines()
            .skip(2)
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>()
            .chunks(5)
            .for_each(|chunk| {
                boards.push(Day04::get_board_from_lines(chunk));
            });

        let marker_board = [[false; 5]; 5];
        let mut boards_marker = Vec::new();

        for _i in 0..boards.len() {
            boards_marker.push(marker_board.clone());
        }

        Ok(Self {
            draw,
            boards,
            boards_marker,
        })
    }

    fn get_board_from_lines(board_str: &[&str]) -> [[u32; 5]; 5] {
        let mut board = [[0u32; 5]; 5];

        board_str.into_iter().enumerate().for_each(|(row, line)| {
            line.split_whitespace()
                .into_iter()
                .enumerate()
                .for_each(|(col, val)| {
                    board[row][col] = val.parse::<u32>().unwrap();
                });
        });

        board
    }

    fn sum_unmarked(&self, board: &[[u32; 5]; 5], marker_board: &[[bool; 5]; 5]) -> u32 {
        let mut sum: u32 = 0;

        for row in 0..5 {
            for col in 0..5 {
                if !marker_board[row][col] {
                    sum += board[row][col];
                }
            }
        }

        sum
    }

    fn iterate_board(
        &self,
        board: &[[u32; 5]; 5],
        marker_board: &mut [[bool; 5]; 5],
        current_draw: &u32,
    ) -> bool {
        let mut row_counter: u32;
        let mut col_counter: Vec<u32> = vec![0, 0, 0, 0, 0];

        for row in 0..5 {
            row_counter = 0;

            for col in 0..5 {
                if board[row][col] == *current_draw {
                    marker_board[row][col] = true;
                }

                if marker_board[row][col] {
                    col_counter[col] += 1;
                    row_counter += 1;
                }

                if col_counter[col] == 5 || row_counter == 5 {
                    return true;
                }
            }
        }

        false
    }
}

impl Day for Day04 {
    fn a(&self) -> Result<()> {
        let mut marker = self.boards_marker.clone();

        'draw: for current_draw in self.draw.iter() {
            for (i, board) in self.boards.iter().enumerate() {
                let bingo = self.iterate_board(board, &mut marker[i], current_draw);

                if bingo {
                    let sum_unmarked = self.sum_unmarked(board, &marker[i]);

                    println!("BINGO! Winner Number: {}", current_draw);
                    println!(
                        "Score: {} ({}*{})",
                        current_draw * sum_unmarked,
                        current_draw,
                        sum_unmarked
                    );

                    break 'draw;
                }
            }
        }

        Ok(())
    }

    fn b(&self) -> Result<()> {
        let mut marker = self.boards_marker.clone();
        let mut winner_boards: Vec<[[u32; 5]; 5]> = Vec::new();
        let mut winner_boards_i: Vec<u32> = Vec::new();

        let mut last_draw = 0;

        'draw: for current_draw in self.draw.iter() {
            for (i, board) in self.boards.iter().enumerate() {
                let bingo = self.iterate_board(board, &mut marker[i], current_draw);

                if bingo && !winner_boards.contains(board) {
                    winner_boards.push(*board);
                    winner_boards_i.push(i as u32);
                }

                if winner_boards.len() == self.boards.len() {
                    last_draw = *current_draw;
                    break 'draw;
                }
            }
        }

        let last_board = winner_boards.last().unwrap();

        let sum_unmarked = self.sum_unmarked(
            last_board,
            &marker[self
                .boards
                .iter()
                .position(|board| board == last_board)
                .unwrap()],
        );

        println!(
            "Score: {} ({}*{})",
            last_draw * sum_unmarked,
            last_draw,
            sum_unmarked
        );

        Ok(())
    }
}
