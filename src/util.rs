use color_eyre::eyre::Result;
use std::path::PathBuf;

use std::fs;

pub trait Day {
    fn a(&self) -> Result<()>;
    fn b(&self) -> Result<()>;
}

pub struct Util {
    input_dir: PathBuf,
}

impl Util {
    pub fn new() -> Util {
        let input_dir = PathBuf::from("./input");

        Self { input_dir }
    }

    pub fn read_input(&self, file_name: &str) -> Result<String> {
        let path = self.input_dir.join(file_name);

        Ok(fs::read_to_string(path).expect("file not found!"))
    }
}
