mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;

use super::Year;
use color_eyre::eyre::Result;

pub fn get() -> Result<Year> {
    let mut year2021 = Year::new(2021);
    year2021.days.push(Box::new(day_01::Day01::new()?));
    year2021.days.push(Box::new(day_02::Day02::new()?));
    year2021.days.push(Box::new(day_03::Day03::new()?));
    year2021.days.push(Box::new(day_04::Day04::new()?));
    year2021.days.push(Box::new(day_05::Day05::new()?));
    year2021.days.push(Box::new(day_06::Day06::new()?));
    year2021.days.push(Box::new(day_07::Day07::new()?));
    year2021.days.push(Box::new(day_08::Day08::new()?));

    Ok(year2021)
}
