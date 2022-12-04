use color_eyre::eyre::Result;
use crate::util::Day;

const DIMENSION: usize = 1000;

#[derive(Debug, Clone)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

pub struct Day05 {
    lines: Vec<Line>,
    matrix: Vec<[usize; DIMENSION]>,
}

impl Day05 {
    pub fn new() -> Result<Day05> {
        let data = Day05::get_input(2021, 5, false)?;
        let lines = data
            .lines()
            .map(|line| Day05::parse_input_line(line))
            .collect::<Vec<Line>>();

        // This was just for finding out max sizes

        // let x_max = lines.iter().max_by(|a, b| {
        //     a.x1.cmp(&b.x1).then(a.x2.cmp(&b.x2))
        // }).unwrap();

        // let y_max = lines.iter().max_by(|a, b| {
        //     a.y1.cmp(&b.y1).then(a.y2.cmp(&b.y2))
        // }).unwrap();

        // dbg!(x_max);
        // dbg!(y_max);

        let mut matrix: Vec<[usize; DIMENSION]> = Vec::new();
        for _i in 0..DIMENSION {
            matrix.push([0usize; DIMENSION]);
        }

        Ok(Self { lines, matrix })
    }

    fn parse_input_line(input_line: &str) -> Line {
        let mut points = input_line.split(" -> ");

        let p1 = points.next().unwrap();
        let mut vals_p1 = p1.split(",");

        let p2 = points.next().unwrap();
        let mut vals_p2 = p2.split(",");

        Line {
            x1: vals_p1.next().unwrap().parse::<usize>().unwrap(),
            y1: vals_p1.next().unwrap().parse::<usize>().unwrap(),
            x2: vals_p2.next().unwrap().parse::<usize>().unwrap(),
            y2: vals_p2.next().unwrap().parse::<usize>().unwrap(),
        }
    }

    // This function is just to work around the fact that Rust ranges with (start > end) are empty
    fn get_range(start: usize, end: usize) -> Vec<usize> {
        let range = if start < end {
            (start..=end).collect::<Vec<usize>>()
        } else {
            (end..=start).rev().collect::<Vec<usize>>()
        };

        range
    }

    fn count_overlaps(matrix: &Vec<[usize; DIMENSION]>) -> i32 {
        let mut overlaps = 0;

        for x in 0..matrix.len() {
            for y in 0..matrix.len() {
                if matrix[x][y] > 1 {
                    overlaps += 1;
                }
            }
        }

        overlaps
    }

    fn _print_matrix(matrix: &Vec<[usize; DIMENSION]>) {
        for x in 0..matrix.len() {
            for y in 0..matrix.len() {
                if matrix[x][y] > 0 {
                    print!("{}", matrix[x][y]);
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn draw_lines_horizontal_vertical(&self, matrix: &mut Vec<[usize; DIMENSION]>) {
        let lines_horizontal_vertical = self
            .lines
            .iter()
            .filter(|line| line.y1 == line.y2 || line.x1 == line.x2)
            .collect::<Vec<&Line>>();

        lines_horizontal_vertical.iter().for_each(|line| {
            if line.x1 == line.x2 {
                let range = Day05::get_range(line.y1, line.y2);
                for y in range {
                    matrix[line.x1][y] += 1;
                }
            } else {
                let range = Day05::get_range(line.x1, line.x2);
                for x in range {
                    matrix[x][line.y1] += 1;
                }
            }
        });
    }

    fn draw_lines_diagonal(&self, matrix: &mut Vec<[usize; DIMENSION]>) {
        let lines_diagonal = self
            .lines
            .iter()
            .filter(|line| line.y1 != line.y2 && line.x1 != line.x2)
            .collect::<Vec<&Line>>();

        lines_diagonal.iter().for_each(|line| {
            let range_x = Day05::get_range(line.x1, line.x2);
            let range_y = Day05::get_range(line.y1, line.y2);

            // Vertical lines always have the same length of X and Y
            for i in 0..range_x.len() {
                let x = range_x[i];
                let y = range_y[i];
                matrix[x][y] += 1;
            }
        });
    }
}

impl Day for Day05 {
    fn a(&self) -> Result<String> {
        let mut matrix = self.matrix.clone();

        self.draw_lines_horizontal_vertical(&mut matrix);

        // Day05::_print_matrix(&matrix);

        let overlaps = Day05::count_overlaps(&matrix);
        let print = format!("Overlaps Horizontal & Vertical: {}", overlaps);

        Ok(print)
    }

    fn b(&self) -> Result<String> {
        let mut matrix = self.matrix.clone();

        self.draw_lines_horizontal_vertical(&mut matrix);
        self.draw_lines_diagonal(&mut matrix);

        // Day05::_print_matrix(&matrix);

        let overlaps = Day05::count_overlaps(&matrix);
        let print = format!("Overlaps all: {}", overlaps);

        Ok(print)
    }

    fn get_title(&self) -> &str {
        "--- Day 5: Hydrothermal Venture ---"
    }
}
