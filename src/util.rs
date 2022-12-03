use color_eyre::eyre::Result;
use std::path::PathBuf;

use std::fs;

pub struct Year {
    pub number: u32,
    pub days: Vec<Box<dyn Day>>,
}

impl Year {
    pub fn new(year: u32) -> Year {
        Self {
            number: year,
            days: Vec::new(),
        }
    }
}

pub trait Day {
    fn a(&self) -> Result<String>;
    fn b(&self) -> Result<String>;
    fn print_title(&self);
    fn print(&self) -> Result<()> {
        println!("");
        self.print_title();
        println!("[A]: {}", self.a()?);
        println!("[B]: {}", self.b()?);
        Ok(())
    }
}

pub struct InputReader {
    input_file_path: PathBuf,
}

impl InputReader {
    pub fn new(year: u32, day: String, use_example: bool) -> InputReader {
        let mut input_file_path = PathBuf::from(format!("src/years/{}/input/", year));
        if use_example {
            input_file_path = input_file_path.join(format!("day_{}_example.txt", day));
        } else {
            input_file_path = input_file_path.join(format!("day_{}.txt", day));
        }

        Self { input_file_path }
    }

    pub fn read(&self) -> Result<String> {
        Ok(fs::read_to_string(&self.input_file_path).expect("File not found!"))
    }
}
