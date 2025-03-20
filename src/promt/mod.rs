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
    New(OutputType),
    Append(OutputType),
}

#[derive(Clone)]
pub enum OutputType {
    Stdout,
    Stderr,
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

    fn prompt_for_input(&mut self) -> Result<bool, Error> {
        let _ = crossterm::terminal::enable_raw_mode();
        let mut stdout = io::stdout();
        let mut tab_pressed = false;

        loop {
            match crossterm::event::read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Char('c')
                        if event.modifiers.contains(event::KeyModifiers::CONTROL) =>
                    {
                        let _ = crossterm::terminal::disable_raw_mode();
                        exit(1);
                    }
                    KeyCode::Tab => {
                        tab_pressed = true;
                        let _ = crossterm::terminal::disable_raw_mode();
                        break;
                    }
                    KeyCode::Enter => {
                        let _ = crossterm::terminal::disable_raw_mode();
                        self.input.push('\n');
                        println!("");
                        break;
                    }
                    KeyCode::Backspace => {
                        if self.input.is_empty() {
                            continue;
                        }
                        self.input.pop();
                        execute!(
                            stdout,
                            crossterm::cursor::MoveLeft(1),
                            crossterm::style::Print(' '),
                            crossterm::cursor::MoveLeft(1)
                        )?
                    }
                    KeyCode::Char(c) => {
                        self.input.push(c);
                        execute!(stdout, crossterm::style::Print(c))?
                    }
                    _ => continue,
                },
                _ => continue,
            }
        }
        return Ok(tab_pressed);
    }

    fn print_leader(&self, symbol: char) {
        print!("{symbol} ");
        io::stdout().flush().unwrap();
    }
}
