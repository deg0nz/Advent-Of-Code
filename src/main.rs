// mod solutions;
mod cli;
mod util;
mod years;

use color_eyre::eyre::Result;
use util::Year;
use years::twenty_21;
use years::twenty_22;

fn main() -> Result<()> {
    color_eyre::install()?;

    // 2021

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

    // 2022

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

    let mut years: Vec<Year> = Vec::new();
    years.push(year2021);
    years.push(year2022);

    let args = cli::parse_args()?;

    if let Some(year) = args.year {

        let year_to_execute = years
            .iter()
            .find(|current_year| current_year.number == year)
            .expect(format!("Year {} not found.", year).as_str());

        year_to_execute.print();

        if let Some(day) = args.day {
            let day_to_execute = year_to_execute
                .days
                .iter()
                .find(|current_day| current_day.get_number() == day)
                .expect(format!("Day {} in year {} not found.", day, year).as_str());

            day_to_execute.run()?;
        } else {
            for day in year_to_execute.days.iter() {
                day.run()?;
            }
        }

        std::process::exit(0);
    }

    if args.all {
        for year in years.iter() {
            year.print();
            for day in year.days.iter() {
                day.run()?;
            }
        }

        std::process::exit(0);
    }


    let year_to_execute = years.last().unwrap();
    year_to_execute.print();
    let day_to_execute = year_to_execute.days.last().unwrap();
    day_to_execute.run()?;

    Ok(())
}
