use std::{path::{Path, PathBuf}, str::FromStr};

use std::fs;

pub struct Util {
    input_dir: PathBuf,
}

impl Util {
    pub fn new() -> Util {
        let input_dir = PathBuf::from("./input");

        Self { input_dir }
    }

    pub fn read_input<T: FromStr>(&self, file_name: &str) -> Vec<Result<T, <T as FromStr>::Err>> {
        let path = self.input_dir.join(file_name);

        dbg!(&path);

        fs::read_to_string(path)
            .expect("file not found!")
            .lines()
            .map(|x| x.parse::<T>())
            .collect()
    }
}
