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
    let result = check_permutation(input_1.trim(), input_2.trim());
}

fn check_permutation(input_1: &str, input_2: &str) -> bool {
    if input_1.len() != input_2.len() {
        return false;
    }
    let mut input_1_map: HashMap<char, i32> = HashMap::new();
    let input_1_chars: Vec<char> = input_1.chars().collect();
    let input_2_chars: Vec<char> = input_2.chars().collect();
    for character in input_1_chars {
        //        input_1_chars.insert(character, 0);
    }
    return true;
}
