mod reverse;

use reverse::reverse_input;
use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("Enter text: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    let reversed = reverse_input(input);
    println!("Reversed: {}", reversed);
}
