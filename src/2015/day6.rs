use std::collections::HashMap;
use std::fs;
pub fn part1(file: String) {
    let values: String = fs::read_to_string(file).expect("Error opening file");
    let instructions: Vec<&str> = values.split_whitespace().map(|x| x.clone()).collect();
    // For the sake of simplicity, we assume if key does not exist. the light is off
    let grid  = HashMap<(u16, u16), bool>::new();
    for instruction in instructions {
        let commands = instruction.split(" ").collect();
    }
}
