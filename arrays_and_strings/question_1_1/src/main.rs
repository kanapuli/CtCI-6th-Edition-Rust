use std::collections::HashMap;
use std::io;
fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error: Please input a string");
    let input = input.trim();
    let lowered_input = input.to_lowercase();
    if is_unique(lowered_input) {
        println!("The string '{}' is unique", input);
    } else {
        println!("The string '{}' is not unique", input);
    }
}

fn is_unique(input: String) -> bool {
    let input_chars = input.chars().collect::<Vec<char>>();
    let mut unique_chars: HashMap<char, i16> = HashMap::new();
    for character in input_chars {
        let exists = unique_chars.contains_key(&character);
        if !exists {
            unique_chars.insert(character, 0);
        } else {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unique() {
        //Positive assertions
        assert_eq!(is_unique("nonunique".to_string()), false);
        assert_eq!(is_unique("first".to_string()), true);
        //Negative assertions
        assert_ne!(is_unique("Testshouldpass".to_string()), true);
        assert_ne!(is_unique("coding".to_string()), false);
    }
}
