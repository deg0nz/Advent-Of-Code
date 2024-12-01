use color_eyre::eyre::Result;

use super::Year;

mod day_01;
mod day_02;
// mod day_03;
// mod day_04;

pub fn get() -> Result<Year> {
    let mut year2023 = Year::new(2023);
    year2023.days.push(Box::new(day_01::Day01::new()?));
    year2023.days.push(Box::new(day_02::Day02::new()?));
    // year2023.days.push(Box::new(day_03::Day03::new()?));
    // year2023.days.push(Box::new(day_04::Day04::new()?));

    Ok(year2023)
}
