use std::collections::HashMap;
use std::fs;
fn get_list_of_coords(start: &str, end: &str) -> Vec<(u16, u16)> {
    let parsed_start: Vec<u16> = start
        .split(",")
        .map(|x| x.parse::<u16>().unwrap())
        .collect();
    let parsed_end: Vec<u16> = end.split(",").map(|x| x.parse::<u16>().unwrap()).collect();
    let mut output: Vec<(u16, u16)> = Vec::new();
    for x in parsed_start[0]..parsed_end[0] + 1 {
        for y in parsed_start[1]..parsed_end[1] + 1 {
            output.push((x, y));
        }
    }
    output
}
pub fn part1(file: String) {
    let values: String = fs::read_to_string(file).expect("Error opening file");
    let instructions: Vec<&str> = values.split("\n").map(|x| x.clone()).collect();
    // For the sake of simplicity, we assume if key does not exist. the light is off
    let mut grid: HashMap<(u16, u16), bool> = HashMap::new();
    for instruction in instructions {
        let commands: Vec<&str> = instruction.split(" ").collect();
        if commands[0] == "turn" {
            let coords = get_list_of_coords(commands[2], commands[4]);
            for coord in coords {
                if commands[1] == "off" {
                    grid.insert(coord, false);
                } else {
                    grid.insert(coord, true);
                }
            }
        } else if commands[0] == "toggle" {
            let coords = get_list_of_coords(commands[1], commands[3]);
            for coord in coords {
                if grid.contains_key(&coord) {
                    let v = grid.get(&coord).unwrap();
                    grid.insert(coord, !v);
                } else {
                    grid.insert(coord, true);
                }
            }
        }
    }
    println!("{:?}", grid.values().filter(|&x| *x).count());
}

pub fn part2(file: String) {
    let values: String = fs::read_to_string(file).expect("Error opening file");
    let instructions: Vec<&str> = values.split("\n").map(|x| x.clone()).collect();
    // For the sake of simplicity, we assume if key does not exist. the light is off
    let mut grid: HashMap<(u16, u16), u128> = HashMap::new();
    for instruction in instructions {
        let commands: Vec<&str> = instruction.split(" ").collect();
        if commands[0] == "turn" {
            let coords = get_list_of_coords(commands[2], commands[4]);
            for coord in coords {
                if commands[1] == "off" {
                    grid.entry(coord)
                        .and_modify(|x| *x -= if *x == 0 { 0 } else { 1 })
                        .or_insert(0);
                } else {
                    grid.entry(coord).and_modify(|x| *x += 1).or_insert(1);
                }
            }
        } else if commands[0] == "toggle" {
            let coords = get_list_of_coords(commands[1], commands[3]);
            for coord in coords {
                grid.entry(coord).and_modify(|x| *x += 2).or_insert(2);
            }
        }
    }
    println!("{:?}", grid.values().sum::<u128>());
}
