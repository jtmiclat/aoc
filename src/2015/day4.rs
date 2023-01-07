use md5;
use std::fs;
pub fn part1(file: String) {
    let value: String = fs::read_to_string(file).expect("Error opening file");
    for i in 0..1000000000 {
        let concatenated = format!("{value}{i}");
        let digest = md5::compute(concatenated);
        let formatted = format!("{:x}", digest);
        if formatted.starts_with("00000") {
            println!("Digest: {:?}", digest);
            println!("Suffix: {i}");
            break;
        }
    }
}

pub fn part2(file: String) {
    let value: String = fs::read_to_string(file).expect("Error opening file");
    for i in 0..1000000000 {
        let concatenated = format!("{value}{i}");
        let digest = md5::compute(concatenated);
        let formatted = format!("{:x}", digest);
        if formatted.starts_with("000000") {
            println!("Digest: {:?}", digest);
            println!("Suffix: {i}");
            break;
        }
    }
}
