use crate::util::Day;
use color_eyre::eyre::Result;

pub struct Day02 {
    input: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    Increase,
    Decrease,
}

impl Day02 {
    pub fn new() -> Result<Day02> {
        let input = Day02::get_input(2024, 2, false)?;

        Ok(Self { input })
    }

    fn levels_are_safe(&self, a: &i32, b: &i32, direction: Direction) -> bool {
        let distance = a - b;

        let current_direction: Direction;

        if distance > 0 {
            current_direction = Direction::Increase;
        } else {
            current_direction = Direction::Decrease
        }

        if current_direction != direction {
            return false;
        }

        if distance == 0 || distance.abs() > 3 {
            return false;
        }

        return true;
    }

    fn check_report(&self, report: &Vec<i32>) -> Option<usize> {
        let direction: Direction;
        let distance = report[0] - report[1];

        if distance <= 0 {
            direction = Direction::Decrease
        } else {
            direction = Direction::Increase
        }

        let mut levels = report.iter().enumerate().peekable();
        let mut unsafe_idx = None;

        while let Some((idx, current)) = levels.next() {
            if let Some((_next_idx, next)) = levels.peek() {
                if let None = unsafe_idx {
                    if !self.levels_are_safe(current, *next, direction.clone()) {
                        unsafe_idx = Some(idx.clone())
                    }
                }
            }
        }

        return unsafe_idx;
    }
}

impl Day for Day02 {
    fn a(&self) -> Result<String> {
        let safe_reports: Vec<bool> = self
            .input
            .lines()
            .map(|report| {
                let report_num: Vec<i32> = report
                    .split_whitespace()
                    .into_iter()
                    .map(|val| val.parse().unwrap())
                    .collect();

                let fail_idx = self.check_report(&report_num);

                return fail_idx == None;
            })
            .filter(|elem| *elem == true)
            .collect();

        Ok(safe_reports.iter().count().to_string())
    }

    fn b(&self) -> Result<String> {
        let safe_reports: Vec<bool> = self
            .input
            .lines()
            .map(|report_str| {
                let report: Vec<i32> = report_str
                    .split_whitespace()
                    .into_iter()
                    .map(|val| val.parse().unwrap())
                    .collect();

                let mut safe = false;

                for i in 0..report.len() {
                    let mut subreport = report.clone();
                    subreport.remove(i);

                    if let None = self.check_report(&subreport) {
                        safe = true;
                    }
                }

                return safe;
            })
            .filter(|elem| *elem == true)
            .collect();

        Ok(safe_reports.iter().count().to_string())
    }

    fn get_title(&self) -> &str {
        "--- Day 2: Red-Nosed Reports ---"
    }
}
