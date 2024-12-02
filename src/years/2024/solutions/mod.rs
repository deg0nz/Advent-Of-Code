use color_eyre::eyre::Result;

use super::Year;

mod day_01;
mod day_02;

pub fn get() -> Result<Year> {
    let mut year2024 = Year::new(2024);
    year2024.days.push(Box::new(day_01::Day01::new()?));
    year2024.days.push(Box::new(day_02::Day02::new()?));
    // year2024.days.push(Box::new(day_03::Day03::new()?));
    // year2024.days.push(Box::new(day_04::Day04::new()?));

    Ok(year2024)
}
