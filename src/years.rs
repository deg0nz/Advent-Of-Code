use color_eyre::Report;

use crate::util::Day;

#[path = "years/2021/solutions/mod.rs"]
pub mod twenty_21;
#[path = "years/2022/solutions/mod.rs"]
pub mod twenty_22;
#[path = "years/2023/solutions/mod.rs"]
pub mod twenty_23;
#[path = "years/2024/solutions/mod.rs"]
pub mod twenty_24;

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

pub fn get() -> Result<Vec<Year>, Report> {
    // ================ 2021 ================

    let mut years: Vec<Year> = Vec::new();
    years.push(twenty_21::get()?);
    years.push(twenty_22::get()?);
    years.push(twenty_23::get()?);
    years.push(twenty_24::get()?);

    Ok(years)
}
