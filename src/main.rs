use std::io::Write;

use crate::reader::{ConsoleReader, Reader};
use crate::reverse::{RealReverser, Reverser};

mod reader;
mod reverse;

fn main_<R: Reverser, RD: Reader>(reverser: R, reader: RD) {
    print!("Enter text: ");
    std::io::stdout().flush().unwrap();
    let input = reader.read_line();
    let reversed = reverser.reverse_input(&input);
    println!("Reversed: {}", reversed);
}

fn main() {
    let reverser = RealReverser;
    let reader = ConsoleReader;
    main_(reverser, reader);
}

#[cfg(test)]
mod tests {
    use mockall::predicate;

    use crate::main_;
    use crate::reader::MockReader;
    use crate::reverse::{MockReverser, RealReverser};

    #[test]
    fn test_reverse_input_using_a_mock_reverser() {
        let mut mock = MockReverser::new();
        let user_input = "hello";
        let reversed = "olleh";
        mock.expect_reverse_input()
            .with(predicate::eq(user_input))
            .once()
            .returning(|_| reversed.to_string());

        let mut reader = MockReader::new();
        reader
            .expect_read_line()
            .returning(|| user_input.to_string());

        main_(mock, reader);
    }
    #[test]
    fn test_reverse_input_using_real_reverser() {
        let mock = RealReverser;
        let user_input = "hello";

        let mut reader = MockReader::new();
        reader
            .expect_read_line()
            .returning(|| user_input.to_string());

        main_(mock, reader);
    }
}
