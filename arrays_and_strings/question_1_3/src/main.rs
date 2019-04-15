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
    println!("{:?}", input_chars);
    for character in input_chars {
        if character == ' ' {
            space_count += 1;
        }
    }
    println!("{:?}", space_count);
}
