use chrono::Datelike;
use std::io::Write;
use std::{env::current_exe, fs::File};

use crate::years::Year;
use color_eyre::{Report, Result as EyreResult};

pub struct Args {
    pub year: Option<u32>,
    pub day: Option<u32>,
    pub all: bool,
    pub new_day: bool,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut year = None;
    let mut day = None;
    let mut new_day: bool = false;
    let mut all = false;
    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Short('n') | Long("new-day") => {
                new_day = true;
            }
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

    Ok(Args {
        year,
        day,
        all,
        new_day,
    })
}

pub fn process_args(years: Vec<Year>) -> Result<(), Report> {
    let args = parse_args()?;
    let current_date = chrono::Utc::now();

    if args.new_day {
        let mut year = current_date.year();
        let mut day = current_date.day();

        if let Some(day_arg) = args.day {
            day = day_arg;
        }

        if let Some(year_arg) = args.year {
            year = year_arg as i32;
        }

        create_day_file(year, day)?;

        return Ok(());
    }

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

fn create_day_file(year: i32, day: u32) -> EyreResult<()> {
    println!("Creating new day file for year {}, day {}", year, day);

    let mut day_str = day.to_string();

    if day < 10 {
        day_str.insert(0, '0');
    }

    let day_template = format!(
        "
use crate::util::Day;
use color_eyre::eyre::Result;

pub struct Day{day_str} {{
    input: String,
}}


impl Day{day_str} {{
    pub fn new() -> Result<Day{day_str}> {{
        let input = Day{day_str}::get_input({year}, {day}, false)?;

        Ok(Self {{ input }})
    }}
}}

impl Day for Day{day_str} {{
    fn a(&self) -> Result<String> {{
        Ok(\"a\".to_string())
    }}

    fn b(&self) -> Result<String> {{
        Ok(\"b\".to_string())
    }}

    fn get_title(&self) -> &str {{
        \"--- Day {day}: FooBar ---\"
    }}
}}
    ",
        day_str = day_str,
        year = year,
        day = day
    );

    let current_path = current_exe()?;
    let path = format!(
        "{}/src/years/{}/solutions/day_{}.rs",
        current_path
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .display(),
        year,
        day_str
    );

    println!("{}", path);

    let mut day_file = File::create(path)?;

    write!(&mut day_file, "{}", day_template)?;

    Ok(())
}
