use clap::Parser;
#[path = "2015/mod.rs"]
mod year_2015;
use year_2015::day1;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    year: u16,
    day: u8,
    part: u8,
    file: String,
}

fn main() {
    let args = Args::parse();
    let year = args.year;
    let day = args.day;
    let part = args.part;
    let file = args.file;
    match (year, day, part) {
        (2015, 1, 1) => day1::solve_2015_day1_1(file),
        _ => println!("No solution found."),
    }
}
