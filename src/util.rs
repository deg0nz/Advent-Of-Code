use color_eyre::eyre::Result;
use regex::Regex;
use std::fs;
use std::path::PathBuf;

pub trait Day {
    fn a(&self) -> Result<String>;
    fn b(&self) -> Result<String>;
    fn get_title(&self) -> &str;

    fn get_input(year: u32, day: u32, use_example: bool) -> Result<String>
    where
        Self: Sized,
    {
        let input = read_input(year, day, use_example)?;
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

pub fn read_input(year: u32, day: u32, use_example: bool) -> Result<String> {
    let mut input_file_path = PathBuf::from(format!("src/years/{}/input/", year));
    if use_example {
        input_file_path = input_file_path.join(format!("day_{}_example.txt", day));
    } else {
        input_file_path = input_file_path.join(format!("day_{}.txt", day));
    }

    Ok(fs::read_to_string(input_file_path)?)
}
