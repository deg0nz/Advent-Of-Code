use std::{collections::HashMap, path::PathBuf};

use crate::util::{self, Day};
use color_eyre::eyre::Result;

pub struct Day07 {
    data: String,
    current_path: PathBuf,
    dir_sizes: HashMap<PathBuf, i64>,
}

impl Day07 {
    pub fn new() -> Result<Day07> {
        let input = util::read_input(2022, 7, false)?;
        Ok(Self {
            data: input,
            current_path: PathBuf::new(),
            dir_sizes: HashMap::new(),
        })
    }

    fn handle_line(&mut self, line: &str) -> Result<()> {
        let line_vec: Vec<&str> = line.split_whitespace().collect();

        match *line_vec.first().unwrap() {
            "$" => {
                let cmd = line_vec[1];

                if cmd == "cd" {
                    let arg = line_vec[2];
                    if arg == ".." {
                        self.current_path.pop();
                    } else {
                        let dir_name = arg.to_string();
                        self.current_path.push(dir_name.clone());
                        self.dir_sizes.insert(self.current_path.clone(), 0);
                    }
                }
            }
            dir_content => {
                if Day07::is_file(dir_content) {
                    let size = i64::from_str_radix(*line_vec.first().unwrap(), 10)?;
                    self.add_size_to_dirs_in_current_path(size);
                }
            }
        }

        Ok(())
    }

    fn add_size_to_dirs_in_current_path(&mut self, size: i64) {
        self.dir_sizes.iter_mut().for_each(|(path, value)| {
            if self.current_path.starts_with(path) {
                *value += size
            }
        })
    }

    fn is_file(line: &str) -> bool {
        line.chars().next().unwrap().is_numeric()
    }

    // Workaround for non-mutable self in Day trait
    fn traverse_fs() -> Result<Self> {
        let mut day_07 = Day07::new()?;

        for line in day_07.data.clone().lines() {
            day_07.handle_line(line)?;
        }

        Ok(day_07)
    }
}

impl Day for Day07 {
    fn a(&self) -> Result<String> {
        let day_07 = Day07::traverse_fs()?;

        let sum_dirs_smaller_than_100k: i64 = day_07
            .dir_sizes
            .iter()
            .filter_map(|(_dir_name, size)| if *size <= 100000 { Some(size) } else { None })
            .sum();

        Ok(format!(
            "Total size of all directories < 100000: {}",
            sum_dirs_smaller_than_100k.to_string()
        ))
    }

    fn b(&self) -> Result<String> {
        let day_07 = Day07::traverse_fs()?;
        let free_space = 70000000 - day_07.dir_sizes.get(&PathBuf::from("/")).unwrap();
        let space_needed = 30000000 - free_space;
        let smallest_folder = day_07
            .dir_sizes
            .iter()
            .filter_map(|(_path, size)| {
                if *size > space_needed {
                    Some(size)
                } else {
                    None
                }
            })
            .min()
            .unwrap();

        Ok(format!(
            "Size of smallest folder to delete for update: {}",
            smallest_folder.to_string()
        ))
    }

    fn get_title(&self) -> &str {
        "--- Day 7: No Space Left On Device ---"
    }
}
