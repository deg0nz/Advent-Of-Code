pub struct Args {
    pub year: Option<u32>,
    pub day: Option<u32>,
    pub all: bool,
}

pub fn parse_args() -> Result<Args, lexopt::Error> {
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
                println!("No arguments mean the las day of the most recent year is executed.");
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Args { year, day, all })
}
