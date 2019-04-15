// Task: Urlify-  replace spaces with %20
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Please input a string");
    replace_spaces(input);
}

fn replace_spaces(input: String) {
    let mut space_count = 0;
    let input_chars: Vec<char> = input.chars().collect();
    for character in input_chars {
        if character == ' ' {
            space_count += 1;
        }
    }
    let target_length = input.len() + space_count * 3;
    let mut tmp_array: [char; target_length] = [' '; target_length];
    println!("{:?}", tmp_array);
}
