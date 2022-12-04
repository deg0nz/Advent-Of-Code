mod cli;
mod util;
mod years;

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    cli::process_args(years::get()?)?;
    Ok(())
}
