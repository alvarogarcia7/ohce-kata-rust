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
    use crate::reverse::MockReverser;

    #[test]
    fn test_reverse_input() {
        let mut mock = MockReverser::new();
        mock.expect_reverse_input()
            .with(predicate::eq("hello"))
            .once()
            .returning(|_| "olleh".to_string());

        let mut reader = MockReader::new();
        reader.expect_read_line().returning(|| "hello".to_string());

        main_(mock, reader);
    }
}
