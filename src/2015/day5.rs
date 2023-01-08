use std::fs;

fn good_string(string: &str) -> bool {
    let bad_substrings: Vec<&str> = vec!["ab", "cd", "pq", "xy"];
    let vowel_num = string
        .split("")
        .filter(|x| ["a", "e", "i", "o", "u"].contains(x))
        .count()
        >= 3;
    let mut previous_c = '\0';
    let mut has_double_char = false;
    let char_vec: Vec<char> = string.chars().collect();
    for c in char_vec {
        if previous_c == c {
            has_double_char = true;
            break;
        } else {
            previous_c = c
        }
    }
    let mut contains_bad_pairs = false;
    for bad_pairs in bad_substrings {
        if string.contains(bad_pairs) {
            contains_bad_pairs = true;
            break;
        }
    }

    vowel_num && has_double_char && !contains_bad_pairs
}
fn good_string2(string: &str) -> bool {
    let char_vec: Vec<char> = string.chars().collect();
    let mut has_repeating_char = false;
    let mut previous_c = '\0';
    let mut previous_c_2 = '\0';
    for c in char_vec {
        if previous_c_2 == c {
            has_repeating_char = true;
            break;
        } else {
            previous_c_2 = previous_c;
            previous_c = c;
        }
    }
    has_repeating_char
}
pub fn part1(file: String) {
    let values: String = fs::read_to_string(file).expect("Error opening file");
    let strings: Vec<&str> = values.split_whitespace().map(|x| x.clone()).collect();

    let num_good_strings = strings.iter().filter(|x| good_string(x)).count();
    println!("Good strings {:?}", num_good_strings)
}

pub fn part2(file: String) {
    let values: String = fs::read_to_string(file).expect("Error opening file");
    let strings: Vec<&str> = values.split_whitespace().map(|x| x.clone()).collect();
    let num_good_strings = strings.iter().filter(|x| good_string2(x)).count();
    println!("Good strings {:?}", num_good_strings)
}
