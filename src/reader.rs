use mockall::automock;

#[automock]
pub trait Reader {
    fn read_line(&self) -> String;
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
