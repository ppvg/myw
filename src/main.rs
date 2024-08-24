mod commands;
mod report;
mod timelog;
mod utils;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap::command!()
        .subcommand(commands::report::define())
        .subcommand(commands::debug::define())
        .get_matches();

    match matches.subcommand() {
        Some(("report", matches)) => commands::report::run(matches)?,
        Some(("debug", matches)) => commands::debug::run(matches)?,
        None => {}
        Some(_) => todo!(),
    }

    Ok(())
}
