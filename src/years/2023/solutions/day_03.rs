use crate::util::Day;
use color_eyre::eyre::Result;
use regex::Regex;

#[derive(Debug)]
struct PartNumber {
    row: usize,
    start: usize,
    end: usize,
    value: u32,
}

pub struct Day03 {
    input: String,
    matrix: Vec<Vec<char>>,
}

impl Day03 {
    pub fn new() -> Result<Day03> {
        let input = Day03::get_input(2023, 3, true)?;
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let mut matrix = vec![vec![' '; width]; height];

        input.lines().enumerate().for_each(|(row, line)| {
            line.chars()
                .into_iter()
                .enumerate()
                .for_each(|(col, c)| matrix[row][col] = c)
        });

        Ok(Self { input, matrix })
    }

    fn part_number_has_adjacent_symbol(&self, num: &PartNumber) -> bool {
        // left from num
        if num.start > 0 {
            if Day03::is_symbol(self.matrix[num.row][num.start - 1]) {
                return true;
            };
        }

        // right from num
        if num.end + 1 < self.matrix[0].len() {
            if Day03::is_symbol(self.matrix[num.row][num.end]) {
                return true;
            };
        }

        // above num
        if num.row > 0 {
            for i in num.start..num.end {
                if Day03::is_symbol(self.matrix[num.row - 1][i]) {
                    return true;
                }
            }
        }

        // under num
        if num.row + 1 < self.matrix.len() {
            for i in num.start..num.end {
                if Day03::is_symbol(self.matrix[num.row + 1][i]) {
                    return true;
                }
            }
        }

        // top left
        if num.start > 0 && num.row > 0 {
            if Day03::is_symbol(self.matrix[num.row - 1][num.start - 1]) {
                return true;
            };
        }

        // top right
        if num.row > 0 && num.end + 1 < self.matrix[0].len() {
            if Day03::is_symbol(self.matrix[num.row - 1][num.end]) {
                return true;
            };
        }

        // bottom left
        if num.start > 0 && num.row + 1 < self.matrix.len() {
            if Day03::is_symbol(self.matrix[num.row + 1][num.start - 1]) {
                return true;
            };
        }

        // bottom right
        if num.row + 1 < self.matrix.len() && num.end + 1 < self.matrix[0].len() {
            if Day03::is_symbol(self.matrix[num.row + 1][num.end]) {
                return true;
            };
        }

        return false;
    }

    fn find_two_adjacent_numbers(&self, row: usize, col: usize) -> Option<Vec<(usize, usize)>> {
        let mut coords: Vec<(usize, usize)> = Vec::new();

        fn add_coords(coords: &mut Vec<(usize, usize)>, coord: (usize, usize)) {
            if coords.len() == 0 {
                coords.push(coord);
            } else {
                let (x1, y1) = coords[0];
                let (x2, y2) = coord;

                if x1.abs_diff(x2) + y1.abs_diff(y2) == 1 {
                    coords.push(coord);
                }
            }
        }

        for r in row - 1..row + 1 {
            for c in col - 1..col + 1 {
                dbg!(r, c);
                if r > 0 && r < self.matrix.len() && c > 0 && c < self.matrix[0].len() {
                    if self.matrix[r][c].is_digit(10) {
                        // add_coords(&mut coords, (r, c));
                        coords.push((r, c));
                    }
                }
            }
        }

        // // left num
        // if col > 0 {
        //     let coord = self.matrix[row][col - 1];
        //     if coord.is_digit(10) {
        //         add_coords(&mut coords, (row, col - 1));
        //     };
        // }

        // // right num
        // if col + 1 < self.matrix[0].len() {
        //     let coord = self.matrix[row][col + 1];
        //     if coord.is_digit(10) {
        //         add_coords(&mut coords, (row, col + 1));
        //     };
        // }

        // // above
        // if row > 0 {
        //     let coord = self.matrix[row - 1][col];
        //     if coord.is_digit(10) {
        //         add_coords(&mut coords, (row - 1, col));
        //     }
        // }

        // // under
        // if row + 1 < self.matrix.len() {
        //     let coord = self.matrix[row + 1][col];
        //     if coord.is_digit(10) {
        //         add_coords(&mut coords, (row + 1, col));
        //     }
        // }

        // // top left
        // if col > 0 && row > 0 {
        //     let coord = self.matrix[row - 1][col - 1];
        //     if coord.is_digit(10) {
        //         add_coords(&mut coords, (row - 1, col - 1));
        //     };
        // }

        // // top right
        // if row > 0 && col + 1 < self.matrix[0].len() {
        //     let coord = self.matrix[row - 1][col + 1];
        //     if coord.is_digit(10) {
        //         add_coords(&mut coords, (row - 1, col + 1));
        //     };
        // }

        // // bottom left
        // if col > 0 && row + 1 < self.matrix.len() - 1 {
        //     let coord = self.matrix[row + 1][col - 1];
        //     if coord.is_digit(10) {
        //         add_coords(&mut coords, (row + 1, col - 1));
        //     };
        // }

        // // bottom right
        // if row + 1 < self.matrix.len() && col + 1 < self.matrix[0].len() {
        //     let coord = self.matrix[row + 1][col + 1];
        //     if coord.is_digit(10) {
        //         add_coords(&mut coords, (row + 1, col + 1));
        //     };
        // }

        dbg!(&coords);

        if coords.len() == 2 {
            return Some(coords);
        }

        return None;
    }

    fn is_symbol(c: char) -> bool {
        !c.is_digit(10) && !c.eq(&'.')
    }
}

impl Day for Day03 {
    fn a(&self) -> Result<String> {
        let re = Regex::new(r"\d+").unwrap();
        let mut values_of_part_numbers_with_adjacents: Vec<u32> = Vec::new();

        self.input.lines().enumerate().for_each(|(row, line)| {
            re.find_iter(line).for_each(|m| {
                let capture_str = m.as_str();
                let part_number = PartNumber {
                    row: row,
                    start: m.start(),
                    end: m.end(),
                    value: capture_str.parse().unwrap(),
                };

                if self.part_number_has_adjacent_symbol(&part_number) {
                    values_of_part_numbers_with_adjacents.push(part_number.value);
                }
            });
        });

        let sum = values_of_part_numbers_with_adjacents.iter().sum::<u32>();

        Ok(sum.to_string())
    }

    fn b(&self) -> Result<String> {
        let re = Regex::new(r"\d+").unwrap();

        let mut sum: u32 = 0;

        self.input.lines().enumerate().for_each(|(row, line)| {
            line.match_indices('*').for_each(|(col, _c)| {
                // dbg!(&col, &row);
                if let Some(coords) = self.find_two_adjacent_numbers(row, col) {
                    let mut gear_ratio: u32 = 1;

                    // dbg!(&coords);

                    coords.iter().for_each(|(num_row, num_col)| {
                        let search_line = self.matrix[*num_col].iter().collect::<String>();
                        re.find_iter(search_line.as_str()).for_each(|m| {
                            if m.start() <= *num_row && m.end() >= *num_row {
                                gear_ratio *= m.as_str().parse::<u32>().unwrap();
                            }
                        })
                    });

                    dbg!(&gear_ratio);

                    sum += gear_ratio;
                }
            });
        });

        Ok(sum.to_string())
    }

    fn get_title(&self) -> &str {
        "--- Day 3: Gear Ratios ---"
    }
}
