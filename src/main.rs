mod util;
mod solutions;

use std::error::Error;
use util::Day;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Day 01:");
    let day_01 = solutions::day_01::Day01::new()?;
    day_01.a();
    day_01.b();

    Ok(())
}
