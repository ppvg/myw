use myw::timelog::Log;
use std::{fs, path::PathBuf};

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_parse_timelog_short(bencher: divan::Bencher) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/bench_short.md");
    let content = fs::read_to_string(&path).unwrap();

    bencher.bench_local(move || {
        Log::parse(&content);
    });
}

#[divan::bench]
fn bench_parse_timelog_long(bencher: divan::Bencher) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/bench_long.md");
    let content = fs::read_to_string(&path).unwrap();

    bencher.bench_local(move || {
        Log::parse(&content);
    });
}

#[divan::bench]
fn bench_parse_timelog_long_scrambled(bencher: divan::Bencher) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/bench_long_scrambled.md");
    let content = fs::read_to_string(&path).unwrap();

    bencher.bench_local(move || {
        Log::parse(&content);
    });
}
