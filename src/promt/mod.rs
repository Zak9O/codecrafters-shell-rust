use std::io::{self, Error, ErrorKind, Write};

use parser::Parser;

mod parser;

#[derive(Clone)]
pub struct Command(pub String, pub Vec<String>);
impl Command {
    fn new() -> Self {
        Self(String::new(), Vec::new())
    }
}

#[derive(Clone)]
pub enum UserInput {
    Command(Command),
    Redirect(Command, RedirectType, String),
}

#[derive(Clone)]
pub enum RedirectType {
    Stdin,
}

pub struct Promt {
    parser: Parser,
}

impl Promt {
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
        }
    }
}

impl Promt {
    pub fn prompt_user(mut self) -> Result<UserInput, Error> {
        self.print_leader('$');
        let input = self.promt_for_input()?;
        if input.trim().is_empty() {
            return Err(Error::new(ErrorKind::InvalidInput, "Input was empty"));
        }
        let result = self.parser.parse(&input);

        match result {
            Err(e) => match e.kind() {
                ErrorKind::InvalidInput => self.get_remaining_user_input(),
                _ => Err(e),
            },
            Ok(result) => Ok(result),
        }
    }

    fn get_remaining_user_input(&mut self) -> Result<UserInput, Error> {
        loop {
            self.print_leader('-');
            let input = self.promt_for_input()?;
            let result = self.parser.parse(&input);
            if result.is_ok() {
                return result;
            }
        }
    }

    fn promt_for_input(&self) -> Result<String, Error> {
        let mut input = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut input)?;
        Ok(input)
    }

    fn print_leader(&self, symbol: char) {
        print!("{symbol} ");
        io::stdout().flush().unwrap();
    }
}
