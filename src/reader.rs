use mockall::automock;
use std::io::Write;

#[automock]
pub trait Reader {
    fn read_line(&self) -> String;
    fn println(&self, message: String) {
        println!("{}", message);
        std::io::stdout().flush().unwrap();
    }
}

pub struct ConsoleReader;

impl Reader for ConsoleReader {
    fn read_line(&self) -> String {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string()
    }
}
