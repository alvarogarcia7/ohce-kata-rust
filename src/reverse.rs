pub fn reverse_input(input: &str) -> String {
    input.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use crate::reverse::reverse_input;

    #[test]
    fn test_reverse_input() {
        assert_eq!(reverse_input("hello"), "olleh");
        assert_eq!(reverse_input("rust"), "tsur");
        assert_eq!(reverse_input(""), "");
    }
}
