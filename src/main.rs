mod util;
mod solutions;

use color_eyre::eyre::Result;

use util::Day;

fn main() -> Result<()> {
    color_eyre::install()?;

    println!("========= Day 01 =========");
    let day_01 = solutions::day_01::Day01::new()?;
    day_01.a()?;
    day_01.b()?;

    println!("========= Day 02 =========");
    let day_02 = solutions::day_02::Day02::new()?;
    day_02.a()?;
    day_02.b()?;

    Ok(())
}
