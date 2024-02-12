use crate::timelog;
use clap::{arg, Command};
use std::{error::Error, fs, path::PathBuf};

pub fn define() -> Command {
    Command::new("debug")
        .visible_alias("d")
        .about("Print parsed TimeSheet data")
        .arg(
            arg!([file] "Path to the file to debug")
                .required(true)
                .value_parser(super::parse_file_path),
        )
}

pub fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
    let file = matches.get_one::<PathBuf>("file").unwrap();
    let content = fs::read_to_string(file)?;
    let timelog::Log(result) = timelog::Log::parse(&content);
    for entry in result {
        println!("{}", entry);
    }
    Ok(())
}
