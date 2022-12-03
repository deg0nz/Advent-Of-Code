// mod solutions;
mod util;
mod years;

use years::twenty_22 as solutions;

use color_eyre::eyre::Result;

use util::Day;

fn main() -> Result<()> {
    color_eyre::install()?;

    // let day_01 = solutions::day_01::Day01::new()?;
    // day_01.a()?;
    // day_01.b()?;

    // let day_02 = solutions::day_02::Day02::new()?;
    // day_02.a()?;
    // day_02.b()?;

    // let day_03 = solutions::day_03::Day03::new()?;
    // day_03.a()?;
    // day_03.b()?;

    // let day_04 = solutions::day_04::Day04::new()?;
    // day_04.a()?;
    // day_04.b()?;

    // let day_05 = solutions::day_05::Day05::new()?;
    // day_05.a()?;
    // day_05.b()?;

    // let day_06 = solutions::day_06::Day06::new()?;
    // day_06.a()?;
    // day_06.b()?;

    // let day_07 = solutions::day_07::Day07::new()?;
    // day_07.a()?;
    // day_07.b()?;

    // 2022

    let day01 = solutions::day_01::Day01::new()?;
    day01.print()?;

    let day02 = solutions::day_02::Day02::new()?;
    day02.print()?;

    let day03 = solutions::day_03::Day03::new()?;
    day03.print()?;

    Ok(())
}
