use clap::Parser;
#[path = "2015/mod.rs"]
mod year_2015;

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
        (2015, 1, 1) => year_2015::day1::part1(file),
        (2015, 1, 2) => year_2015::day1::part2(file),
        _ => println!("No solution found."),
    }
}
