use std::collections::HashMap;
use std::io;

fn main() {
    let mut input_1 = String::new();
    let mut input_2 = String::new();
    println!("Enter the first string");
    io::stdin()
        .read_line(&mut input_1)
        .expect("Invalid first string");
    println!("Enter the second string");
    io::stdin()
        .read_line(&mut input_2)
        .expect("Invalid second string");
    if check_permutation(input_1.trim(), input_2.trim()) {
        println!(
            "The strings '{0}' is a permutation of '{1}'",
            input_1.trim(),
            input_2.trim()
        );
    } else {
        println!(
            "The strings '{0}' is different from '{1}' and permutation does not exist",
            input_1.trim(),
            input_2.trim()
        );
    }
}

fn check_permutation(input_1: &str, input_2: &str) -> bool {
    if input_1.len() != input_2.len() {
        return false;
    }
    let mut input_1_map: HashMap<char, i32> = HashMap::new();
    let mut input_2_map: HashMap<char, i32> = HashMap::new();
    let input_1_chars: Vec<char> = input_1.chars().collect();
    let input_2_chars: Vec<char> = input_2.chars().collect();
    for character in input_1_chars {
        match input_1_map.get(&character) {
            None => input_1_map.insert(character, 1),
            Some(s) => input_1_map.insert(character, s + 1),
        };
    }
    for character in input_2_chars {
        match input_2_map.get(&character) {
            None => input_2_map.insert(character, 1),
            Some(s) => input_2_map.insert(character, s + 1),
        };
    }
    for (key, val) in input_1_map.iter() {
        match input_2_map.get(&key) {
            None => return false,
            Some(v) => {
                if v != val {
                    return false;
                }
            }
        };
    }
    return true;
}
