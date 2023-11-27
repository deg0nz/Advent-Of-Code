use color_eyre::Report;

use crate::util::Day;

#[path = "years/2021/solutions/mod.rs"]
pub mod twenty_21;
#[path = "years/2022/solutions/mod.rs"]
pub mod twenty_22;
#[path = "years/2023/solutions/mod.rs"]
pub mod twenty_23;

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

    let mut year2021 = Year::new(2021);
    year2021
        .days
        .push(Box::new(twenty_21::day_01::Day01::new()?));
    year2021
        .days
        .push(Box::new(twenty_21::day_02::Day02::new()?));
    year2021
        .days
        .push(Box::new(twenty_21::day_03::Day03::new()?));
    year2021
        .days
        .push(Box::new(twenty_21::day_04::Day04::new()?));
    year2021
        .days
        .push(Box::new(twenty_21::day_05::Day05::new()?));
    year2021
        .days
        .push(Box::new(twenty_21::day_06::Day06::new()?));
    year2021
        .days
        .push(Box::new(twenty_21::day_07::Day07::new()?));
    year2021
        .days
        .push(Box::new(twenty_21::day_08::Day08::new()?));

    // ================ 2022 ================

    let mut year2022 = Year::new(2022);
    year2022
        .days
        .push(Box::new(twenty_22::day_01::Day01::new()?));
    year2022
        .days
        .push(Box::new(twenty_22::day_02::Day02::new()?));
    year2022
        .days
        .push(Box::new(twenty_22::day_03::Day03::new()?));
    year2022
        .days
        .push(Box::new(twenty_22::day_04::Day04::new()?));
    year2022
        .days
        .push(Box::new(twenty_22::day_05::Day05::new()?));
    year2022
        .days
        .push(Box::new(twenty_22::day_06::Day06::new()?));
    year2022
        .days
        .push(Box::new(twenty_22::day_07::Day07::new()?));
    year2022
        .days
        .push(Box::new(twenty_22::day_08::Day08::new()?));
    year2022
        .days
        .push(Box::new(twenty_22::day_09::Day09::new()?));
    year2022
        .days
        .push(Box::new(twenty_22::day_10::Day10::new()?));

    // ================ 2023 ================

    let mut year2023 = Year::new(2023);
    year2023.days.push(Box::new(twenty_23::day_01::Day01::new()?));

    let mut years: Vec<Year> = Vec::new();
    years.push(year2021);
    years.push(year2022);
    years.push(year2023);

    Ok(years)
}
