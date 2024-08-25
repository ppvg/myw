use crate::report::{Fill, Report};
use crate::timelog;
use clap::{arg, Command};
use std::{error::Error, fs, path::PathBuf};

pub fn define() -> Command {
    Command::new("report")
        .visible_alias("r")
        .about("Report sum per day and sum per project")
        .arg(
            arg!([file] "Path to the file to report on")
                .required(true)
                .value_parser(super::parse_file_path),
        )
}

pub fn run(matches: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
    let file = matches.get_one::<PathBuf>("file").unwrap();
    let content = fs::read_to_string(file)?;
    let log = timelog::Log::parse(&content);
    let reports = Report::by_date_by_project(&log, Fill::Padded);
    for report in reports {
        println!("{}", report.text());
    }
    println!("{}", Report::total(&log).text());
    Ok(())
}
