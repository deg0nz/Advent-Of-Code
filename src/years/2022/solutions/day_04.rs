use crate::util::Day;
use crate::util::InputReader;
use color_eyre::eyre::Result;

pub struct Day04 {
    data: String,
}

impl Day04 {
    pub fn new() -> Result<Day04> {
        let util = InputReader::new(2022, "04".to_string(), false);
        let input = util.read()?;

        Ok(Self { data: input })
    }

    fn parse_line(line: &str) -> ((i32, i32), (i32, i32)) {
        let sections = line.split(",").collect::<Vec<&str>>();
        let left = Day04::parse_section(sections[0]);
        let right = Day04::parse_section(sections[1]);

        (left, right)
    }

    fn parse_section(section: &str) -> (i32, i32) {
        let parsed = section
            .split("-")
            .map(|s| i32::from_str_radix(s, 10).expect("Could not parse i32"))
            .collect::<Vec<i32>>();

        let tuple: (i32, i32) = (parsed[0], parsed[1]);

        tuple
    }

    fn section_includes_the_other(sections: ((i32, i32), (i32, i32))) -> bool {
        let (left, right) = sections;

        let bigger;
        let smaller;

        // Determine which range is bigger,
        // the bigger range needs to be substracted from smaller range
        if (left.1 - left.0) >= (right.1 - right.0) {
            bigger = left;
            smaller = right;
        } else {
            bigger = right;
            smaller = left;
        }

        (smaller.0 >= bigger.0) && (smaller.1 <= bigger.1)
    }

    fn sections_overlap(sections: ((i32, i32), (i32, i32))) -> bool {
        let (left, right) = sections;

        (left.0 >= right.0) && (left.0 <= right.1)
            || (left.1 >= right.0) && (left.1 <= right.1)
            || (right.0 >= left.0) && (right.0 <= left.1)
            || (right.1 >= left.0) && (right.1 <= left.1)
    }
}

impl Day for Day04 {
    fn a(&self) -> Result<String> {
        let mut num_includes = 0;

        self.data.lines().for_each(|line| {
            if Day04::section_includes_the_other(Day04::parse_line(line)) {
                num_includes += 1;
            }
        });

        Ok(num_includes.to_string())
    }

    fn b(&self) -> Result<String> {
        let mut num_overlaps = 0;

        self.data.lines().for_each(|line| {
            if Day04::sections_overlap(Day04::parse_line(line)) {
                num_overlaps += 1;
            }
        });

        Ok(num_overlaps.to_string())
    }

    fn print_title(&self) {
        println!("--- Day 4: Camp Cleanup ---")
    }
}
