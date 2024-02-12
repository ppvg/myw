use crate::report;
use crate::timelog;
use clap::{arg, Command};
use colored::Colorize;
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
    println!("{}", "By date".bold());
    let hours_by_date = report::by_date(&log, report::Fill::Padded);
    for item in &hours_by_date {
        println!("{}: {}", item.0, format_hours(&item.1));
    }
    println!("{}", "By project".bold());
    let hours_by_project = report::by_project(&log);
    for item in &hours_by_project {
        println!("{}: {}", item.0, format_hours(&item.1));
    }
    let total_hours = report::sum(&hours_by_date);
    println!("{}: {}", "Total".bold(), format_hours(&total_hours));
    Ok(())
}

fn format_hours(td: &chrono::TimeDelta) -> String {
    format!(
        "{}",
        ((td.num_minutes() as f32) / 60.0 * 100.0).round() / 100.0
    )
}
