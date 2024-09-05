use mockall::automock;

#[automock]
pub trait ReverserT {
    fn reverse_input(&self, input: &str) -> String;
}

pub struct ReverserImpl;

impl ReverserT for ReverserImpl {
    fn reverse_input(&self, input: &str) -> String {
        input.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::reverse::{ReverserImpl, ReverserT};

    #[test]
    fn test_reverse_input() {
        assert_eq!(reverse_input("hello"), "olleh");
        assert_eq!(reverse_input("rust"), "tsur");
        assert_eq!(reverse_input(""), "");
    }

    fn reverse_input(p0: &str) -> String {
        ReverserImpl.reverse_input(p0)
    }
}
