use std::fs;
pub fn part1(file: String) {
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

pub fn part2(file: String) {
    let values: String = fs::read_to_string(file).expect("Error opening file");
    let mut floor = 0;
    let characters: Vec<char> = values.chars().collect();
    for (idx, c) in characters.iter().enumerate() {
        match c {
            '(' => floor = floor + 1,
            ')' => floor = floor - 1,
            _ => (),
        }
        if floor < 0 {
            println!("Step Number {:?}", idx + 1);
            break;
        }
    }
}
