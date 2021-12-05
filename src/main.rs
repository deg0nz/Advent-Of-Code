mod util;
mod solutions;

use std::error::Error;

use util::Util;

fn main() -> Result<(), Box<dyn Error>> {
    let day_01 = solutions::day_01::Day01::new()?;
    day_01.a();

    Ok(())
}
