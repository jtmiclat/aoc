use std::fs;
pub fn solve_2015_day1_1(file: String) {
    let values: String = fs::read_to_string(file).expect("Error opening file");
    let mut floor = 0;
    let characters: Vec<char> = values.chars().collect();
    for c in characters {
        match c {
            '(' => floor = floor + 1,
            ')' => floor = floor - 1,
            _ => (),
        }
    }
    println!("Floor {:?}", floor)
}
