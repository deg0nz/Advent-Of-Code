use color_eyre::eyre::Result;

use crate::util::Day;

use super::super::util::Util;

#[derive(Debug, Clone)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

pub struct Day05 {
    lines: Vec<Line>,
    matrix: Vec<[u32; 1000]>,
}

impl Day05 {
    pub fn new() -> Result<Day05> {
        println!("");
        println!("========= Day 05: Hydrothermal Venture =========");

        let util = Util::new();
        let data = util.read_input("day_05.txt")?;
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

        let mut matrix: Vec<[u32; 1000]> = Vec::new();
        for _i in 0..1000 {
            matrix.push([0u32; 1000]);
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

    fn get_range(start: usize, end: usize) -> Vec<usize> {
        let range = if start < end {
            (start..=end).collect::<Vec<usize>>()
        } else {
            (end..=start).collect::<Vec<usize>>()
        };

        range
    }
}

impl Day for Day05 {
    fn a(&self) -> Result<()> {
        let mut overlaps = 0;
        let mut matrix = self.matrix.clone();

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

        for x in 0..matrix.len() {
            for y in 0..matrix.len() {
                if matrix[x][y] > 1 {
                    overlaps += 1;
                }
            }
        }

        println!("Overlaps: {}", overlaps);

        Ok(())
    }

    fn b(&self) -> Result<()> {
        todo!()
    }
}
