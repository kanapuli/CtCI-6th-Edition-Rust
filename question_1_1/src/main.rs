use std::collections::HashMap;

fn main() {
    let input = "Rustlangr";
    let lowered_input = input.to_lowercase();
    let input_chars = lowered_input.chars().collect::<Vec<char>>();
    let mut unique_chars: HashMap<char, i16> = HashMap::new();
    for character in input_chars {
        let exists = unique_chars.contains_key(&character);
        if !exists {
            unique_chars.insert(character, 0);
        } else {
            println!(
                "The value {0} is duplicate and the string '{1}' is not unique",
                character, input
            );
            return;
        }
    }
    println!("The string '{}' is unique", input);
}
