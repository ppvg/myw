use myw::{
    report::{Fill, Report},
    timelog::Log,
};
use std::{fs, path::PathBuf};

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_report_by_date_short(bencher: divan::Bencher) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/bench_short.md");
    let content = fs::read_to_string(&path).unwrap();
    let log = Log::parse(&content);

    bencher.bench_local(move || {
        Report::by_date_by_project(&log, Fill::Padded);
    });
}

#[divan::bench]
fn bench_report_by_date_long(bencher: divan::Bencher) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/bench_long.md");
    let content = fs::read_to_string(&path).unwrap();
    let log = Log::parse(&content);

    bencher.bench_local(move || {
        Report::by_date_by_project(&log, Fill::Padded);
    });
}

#[divan::bench]
fn bench_report_by_date_scrambled(bencher: divan::Bencher) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/bench_long_scrambled.md");
    let content = fs::read_to_string(&path).unwrap();
    let log = Log::parse(&content);

    bencher.bench_local(move || {
        Report::by_date_by_project(&log, Fill::Padded);
    });
}
