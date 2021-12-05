use color_eyre::eyre::Result;

use crate::util::{Util, Day};

pub struct Day03 {
    data: String,
}

impl Day03 {
    pub fn new() -> Result<Day03> {
        let util = Util::new();
        let data = util.read_input("day_03.txt")?;

        Ok(Self { data })
    }
}

impl Day for Day03 {
    fn a(&self) -> Result<()> {
        let line_len = self.data.lines().next().unwrap().len();
        let mut bits_count: Vec<(u32, u32)> = Vec::new();
        let mut gamma_bits = String::new();

        for _n in 0..line_len {
            bits_count.push((0, 0));
        }
        
        self.data.lines().for_each(|line| {
            let chars = line.chars();
            chars.enumerate().for_each(|(i, char)| {
                if char == '0' {
                    bits_count[i].0 += 1;
                } else if char == '1' {
                    bits_count[i].1 += 1;
                }
            });
        });

        for count in bits_count {
            if count.0 > count.1 {
                gamma_bits.push('0');
            } else {
                gamma_bits.push('1');
            }
        }

        let gamma = u32::from_str_radix(gamma_bits.as_str(), 2)?;
        let epsilon = !gamma ^ 0xFFFFF000;

        println!("Gamma: {:b}", gamma);
        println!("Epsilon: {:b}", epsilon);
        // Power consumption: 1131506
        println!("Power consumption: {}", gamma * epsilon);

        Ok(())
    }

    fn b(&self) -> Result<()> {
        todo!()
    }
}