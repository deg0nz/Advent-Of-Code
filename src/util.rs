use color_eyre::eyre::Result;
use regex::Regex;
use std::fs;
use std::path::PathBuf;

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

    pub fn print(&self) {
        println!("");
        println!("==== {} ====", self.number);
        println!("");
    }
}

pub trait Day {
    fn a(&self) -> Result<String>;
    fn b(&self) -> Result<String>;
    fn get_title(&self) -> &str;
    fn get_input(year: u32, day: u32, use_example: bool) -> Result<String>
    where
        Self: Sized,
    {
        let util = InputReader::new(year, day, use_example);
        let input = util.read()?;

        Ok(input)
    }
    fn get_number(&self) -> u32 {
        let re = Regex::new(r"\d+").unwrap();
        let title = self.get_title();
        let number_str = re
            .captures(title)
            .expect("Couldn't find day number in title")
            .get(0)
            .unwrap()
            .as_str();
        u32::from_str_radix(number_str, 10).unwrap()
    }
    fn run(&self) -> Result<()> {
        println!("");
        println!("{}", self.get_title());
        println!("");
        println!("[A]: {}", self.a()?);
        println!("[B]: {}", self.b()?);
        Ok(())
    }
}

pub struct InputReader {
    input_file_path: PathBuf,
}

impl InputReader {
    pub fn new(year: u32, day: u32, use_example: bool) -> InputReader {
        let mut input_file_path = PathBuf::from(format!("src/years/{}/input/", year));
        if use_example {
            input_file_path = input_file_path.join(format!("day_{}_example.txt", day));
        } else {
            input_file_path = input_file_path.join(format!("day_{}.txt", day));
        }

        Self { input_file_path }
    }

    pub fn read(&self) -> Result<String> {
        Ok(fs::read_to_string(&self.input_file_path)?)
    }
}
