mod reverse;

use crate::reverse::{RealReverser, Reverser};
use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("Enter text: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    let reversed = RealReverser.reverse_input(input);
    println!("Reversed: {}", reversed);
}

#[cfg(test)]
mod tests {
    use mockall::{automock, predicate, predicate::*};

    #[automock]
    trait Reverser {
        fn reverse_input(&self, input: &str) -> String;
    }

    #[test]
    fn test_reverse_input() {
        let mut mock = MockReverser::new();
        mock.expect_reverse_input()
            .with(predicate::eq("hello"))
            .returning(|_| "olleh".to_string());

        assert_eq!(mock.reverse_input("hello"), "olleh");
    }
}
