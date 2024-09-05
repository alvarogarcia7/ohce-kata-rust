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

#[cfg(test)]
mod tests {
    use super::reverse::reverse_input;

    #[test]
    fn test_reverse_input() {
        assert_eq!(reverse_input("hello"), "olleh");
        assert_eq!(reverse_input("rust"), "tsur");
        assert_eq!(reverse_input(""), "");
    }
}
