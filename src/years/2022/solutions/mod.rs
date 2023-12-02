mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;

use super::Year;
use color_eyre::eyre::Result;

pub fn get() -> Result<Year> {
    let mut year2022 = Year::new(2022);
    year2022.days.push(Box::new(day_01::Day01::new()?));
    year2022.days.push(Box::new(day_02::Day02::new()?));
    year2022.days.push(Box::new(day_03::Day03::new()?));
    year2022.days.push(Box::new(day_04::Day04::new()?));
    year2022.days.push(Box::new(day_05::Day05::new()?));
    year2022.days.push(Box::new(day_06::Day06::new()?));
    year2022.days.push(Box::new(day_07::Day07::new()?));
    year2022.days.push(Box::new(day_08::Day08::new()?));
    year2022.days.push(Box::new(day_09::Day09::new()?));
    year2022.days.push(Box::new(day_10::Day10::new()?));

    Ok(year2022)
}
