use crate::reader::{Console, ConsoleReader};
use crate::reverse::{ReverserImpl, ReverserT};

mod reader;
mod reverse;

fn main_<R: ReverserT, C: Console>(reverser: R, console: C) {
    console.println("Enter text:".to_string());
    let input = console.read_line();
    let reversed = reverser.reverse_input(&input);
    console.println(format!("Reversed: {}", reversed));
}

// Entry point for the application
fn main() {
    main_(ReverserImpl, ConsoleReader);
}

#[cfg(test)]
mod tests {
    use mockall::predicate;

    use crate::main_;
    use crate::reader::MockConsole;
    use crate::reverse::{MockReverserT, ReverserImpl};

    #[test]
    fn test_reverse_input_using_a_mock_reverser() {
        let mut mock_reverser = MockReverserT::new();
        let user_input = "hello";
        let reversed = "olleh";
        mock_reverser
            .expect_reverse_input()
            .with(predicate::eq(user_input))
            .once()
            .returning(|_| reversed.to_string());

        let mut console = MockConsole::new();
        console
            .expect_read_line()
            .returning(|| user_input.to_string());

        console
            .expect_println()
            .once()
            .with(predicate::eq("Enter text:".to_string()))
            .returning(|_| ());
        console
            .expect_println()
            .once()
            .with(predicate::eq("Reversed: olleh".to_string()))
            .returning(|_| ());

        main_(mock_reverser, console);
    }
    #[test]
    fn test_reverse_input_using_a_real_reverser() {
        let user_input = "hello";

        let mut console = MockConsole::new();
        console
            .expect_read_line()
            .returning(|| user_input.to_string());

        console
            .expect_println()
            .once()
            .with(predicate::eq("Enter text:".to_string()))
            .returning(|_| ());
        console
            .expect_println()
            .once()
            .with(predicate::eq("Reversed: olleh".to_string()))
            .returning(|_| ());

        main_(ReverserImpl, console);
    }
}
