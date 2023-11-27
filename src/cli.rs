use color_eyre::Report;

use crate::years::Year;

pub struct Args {
    pub year: Option<u32>,
    pub day: Option<u32>,
    pub all: bool,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut year = None;
    let mut day = None;
    let mut all = false;
    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Short('y') | Long("year") => {
                year = Some(parser.value()?.parse()?);
            }
            Short('d') | Long("day") => {
                day = Some(parser.value()?.parse()?);
            }
            Short('a') | Long("all") => {
                all = true;
                day = None;
                year = None;
            }
            Short('h') | Long("help") => {
                println!("Usage: aoc [-y|--year] NUM [-d|--day] NUM [-a|--all]");
                println!("If --year is called without --day, all days of that year are executed.");
                println!("If --all is executed, --year and --day are ignored.");
                println!("No arguments mean the last day of the most recent year is executed.");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args { year, day, all })
}

pub fn process_args(years: Vec<Year>) -> Result<(), Report> {
    let args = parse_args()?;

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
