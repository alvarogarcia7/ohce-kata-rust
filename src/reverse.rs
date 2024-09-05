pub trait Reverser {
    fn reverse_input(&self, input: &str) -> String;
}

pub struct RealReverser;

impl Reverser for RealReverser {
    fn reverse_input(&self, input: &str) -> String {
        input.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::reverse::{RealReverser, Reverser};

    #[test]
    fn test_reverse_input() {
        assert_eq!(reverse_input("hello"), "olleh");
        assert_eq!(reverse_input("rust"), "tsur");
        assert_eq!(reverse_input(""), "");
    }

    fn reverse_input(p0: &str) -> String {
        RealReverser.reverse_input(p0)
    }
}
