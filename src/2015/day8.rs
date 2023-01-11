use std::fs;
fn count_escaped_chars(string: &str) -> usize {
    let value: Vec<char> = (string[1..string.len() - 1].to_string()).chars().collect();
    let mut c = 0;
    let mut processed: String = String::new();
    while c < value.len() {
        if value[c] == '\\' {
            c += 1;
            match value[c] {
                '\\' => {
                    processed.push('\\');
                    c += 1;
                }
                '"' => {
                    processed.push('"');
                    c += 1;
                }
                'x' => {
                    processed.push('~');
                    c += 3;
                }
                _ => panic!("at the discor"),
            }
        } else {
            processed.push(value[c] as char);
            c += 1;
        }
    }
    string.len() - processed.len()
}
fn count_escaped_chars_2(value: &str) -> i16 {
    let final_processed = format!("\"{}\"", value.escape_default().to_string());
    println!("{} -> {}", value, final_processed);
    final_processed.len() as i16 - value.len() as i16
}
pub fn part1(file: String) {
    let value: String = fs::read_to_string(file).expect("Error opening file");
    let strings: Vec<&str> = value.split("\n").map(|x| x.clone()).collect();

    let total: usize = strings.iter().map(|x| count_escaped_chars(x)).sum();
    println!("Total: {:?}", total)
}

pub fn part2(file: String) {
    let value: String = fs::read_to_string(file).expect("Error opening file");
    let strings: Vec<&str> = value.split("\n").map(|x| x.clone()).collect();

    let total: i16 = strings.iter().map(|x| count_escaped_chars_2(x)).sum();
    println!("Total: {:?}", total)
}
