use std::error::Error;

use super::super::util::Util;

pub struct Day01{
    data: Vec<u64>,

}

impl Day01 {
    pub fn new() -> Result<Day01, Box<dyn Error>> {
        let util = Util::new();
        let input = util.read_input::<u64>("day_01.txt");

        let data = input.iter().map(|x| x.to_owned().unwrap()).collect();

        Ok(Self {
            data
        })
    }

    pub fn a(&self){
        // dbg!(&self.data);
        let mut increase_counter: u64 = 0;

        for (i, item) in self.data.iter().enumerate() {
            if i < self.data.len() - 1 {
                if *item < self.data[i + 1] {
                    increase_counter += 1;
                }
            }
        }

        print!("Depth: {}", increase_counter);
    }
}