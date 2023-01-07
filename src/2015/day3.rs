use std::collections::HashMap;
use std::fs;
fn move_santa(c: char, coords: (i32, i32)) -> (i32, i32) {
    match c {
        '^' => (coords.0 + 1, coords.1),
        '>' => (coords.0, coords.1 + 1),
        'v' => (coords.0 - 1, coords.1),
        '<' => (coords.0, coords.1 - 1),
        _ => (coords.0, coords.1),
    }
}
pub fn part1(file: String) {
    let values: String = fs::read_to_string(file).expect("Error opening file");
    let characters: Vec<char> = values.chars().collect();
    let mut coords = (0, 0);
    let mut coords_mapping: HashMap<(i32, i32), i32> = HashMap::new();
    coords_mapping.insert(coords, 1);
    for c in characters {
        let new_coords = move_santa(c, coords);
        if coords_mapping.contains_key(&new_coords) {
            //
            let val = coords_mapping
                .get(&new_coords)
                .expect("Error pulling value")
                + 1;
            coords_mapping.insert(new_coords, val);
        } else {
            coords_mapping.insert(new_coords, 1);
        }
        coords = new_coords;
    }
    println!("Number of houses {:?}", coords_mapping.len())
}

pub fn part2(file: String) {
    let values: String = fs::read_to_string(file).expect("Error opening file");
    let characters: Vec<char> = values.chars().collect();
    let mut santa_coords = (0, 0);
    let mut robot_coords = (0, 0);
    let mut coords_mapping: HashMap<(i32, i32), i32> = HashMap::new();
    coords_mapping.insert(santa_coords, 2);

    for (idx, c) in characters.iter().enumerate() {
        let coords = if idx % 2 == 0 {
            santa_coords
        } else {
            robot_coords
        };
        let new_coords = move_santa(*c, coords);
        if coords_mapping.contains_key(&new_coords) {
            //
            let val = coords_mapping
                .get(&new_coords)
                .expect("Error pulling value")
                + 1;
            coords_mapping.insert(new_coords, val);
        } else {
            coords_mapping.insert(new_coords, 1);
        }
        if idx % 2 == 0 {
            santa_coords = new_coords;
        } else {
            robot_coords = new_coords;
        }
    }
    println!("Number of houses {:?}", coords_mapping.len())
}
