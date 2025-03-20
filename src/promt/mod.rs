use std::{
    io::{self, Error, ErrorKind, Write},
    process::exit,
};

use crossterm::{
    event::{Event, KeyCode, KeyModifiers},
    execute,
};
use parser::Parser;

use crate::cmd::BUILTINS;

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
    input: Vec<String>,
}

impl Promt {
    pub fn new() -> Self {
        Self {
            input: vec![String::new()],
        }
    }

    fn auto_complete(&self) -> String {
        let token = self.token();
        let candidates: Vec<&str> = BUILTINS
            .iter()
            .filter(|&x| x.starts_with(&*token))
            .map(|x| *x)
            .collect();
        let mut added_letters = String::new();
        if candidates.len() == 1 {
            let candidate = candidates[0];
            let (start, end) = (token.len(), candidate.len());
            added_letters.push_str(&candidate[start..end]);
            added_letters.push(' ');
        }

        added_letters
    }

    fn token(&self) -> &String {
        self.input.last().unwrap()
    }

    fn token_mut(&mut self) -> &mut String {
        let len = self.input.len();
        self.input.get_mut(len - 1).unwrap()
    }

    fn input_as_str(&self) -> String {
        self.input.join("")
    }

    fn parse(&self) -> Result<UserInput, Error> {
        let parser = Parser::new();
        parser.parse(&self.input_as_str())
    }

    fn promt(mut self, leader: &str) -> Result<UserInput, Error> {
        self.print_leader(leader);
        let tab_pressed = self.prompt_for_input()?;

        if self.is_empty_input() {
            return Err(Error::new(ErrorKind::InvalidInput, "Input was empty"));
        }

        if tab_pressed {
            let letters = self.auto_complete();
            let token = self.token_mut();
            token.push_str(&letters);
            return self.promt(&letters);
        }

        let result = self.parse();

        match result {
            Err(e) => match e.kind() {
                ErrorKind::InvalidInput => self.promt("- "),
                _ => Err(e),
            },
            Ok(result) => Ok(result),
        }
    }

    pub fn prompt_user(self) -> Result<UserInput, Error> {
        self.promt("$ ")
    }

    fn is_empty_input(&self) -> bool {
        self.input.len() == 1 && self.input[0].trim().is_empty()
    }

    fn prompt_for_input(&mut self) -> Result<bool, Error> {
        let mut input = String::new();
        let _ = crossterm::terminal::enable_raw_mode();
        let mut stdout = io::stdout();
        let mut tab_pressed = false;

        loop {
            match crossterm::event::read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => {
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
                        input.push('\n');
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
                        input.push(c);
                        execute!(stdout, crossterm::style::Print(c))?
                    }
                    _ => continue,
                },
                _ => continue,
            }
        }
        self.input.push(input);
        return Ok(tab_pressed);
    }

    fn print_leader(&self, leader: &str) {
        print!("{leader}");
        io::stdout().flush().unwrap();
    }
}
