use std::io::{self, Error, ErrorKind, Write};

pub struct Parser {
    cmd: String,
    args: Vec<String>,
    current_token: Vec<u8>,
    is_inside_apo: bool,
    is_indisde_dapo: bool,
    is_first_char: bool,
    is_escaped: bool,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            cmd: String::new(),
            args: Vec::new(),
            current_token: Vec::new(),
            is_inside_apo: false,
            is_indisde_dapo: false,
            is_first_char: true,
            is_escaped: false,
        }
    }

    pub fn get_user_input(mut self) -> Result<(String, Vec<String>), Error> {
        self.print_leader('$');
        let input = self.promt_for_input()?;
        if input.trim().is_empty() {
            return Err(Error::new(ErrorKind::InvalidInput, "Input was empty"));
        }
        let is_valid = self.parse_input(&input);
        if !is_valid {
            self.get_remaining_user_input()?;
        }
        Ok((self.cmd, self.args))
    }

    fn get_remaining_user_input(&mut self) -> Result<(), Error> {
        loop {
            self.print_leader('-');
            let input = self.promt_for_input()?;
            let is_valid = self.parse_input(&input);
            if is_valid {
                break;
            }
        }
        Ok(())
    }

    fn promt_for_input(&self) -> Result<String, Error> {
        let mut input = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut input)?;
        Ok(input)
    }

    fn is_in_block(&self) -> bool {
        self.is_inside_apo || self.is_indisde_dapo
    }

    fn parse_input(&mut self, input: &str) -> bool {
        for ele in input.as_bytes() {
            match ele {
                _ if self.is_escaped => {
                    self.current_token.push(*ele);
                    self.is_escaped = false;
                }
                b'\\' if !self.is_in_block() => self.is_escaped = true,
                b'\'' if !self.is_indisde_dapo => self.is_inside_apo = !self.is_inside_apo,
                b'\"' if !self.is_inside_apo => self.is_indisde_dapo = !self.is_indisde_dapo,
                b' ' | b'\n' if !self.is_in_block() && !self.is_first_char => {
                    self.is_first_char = true;
                    let current_token_str = String::from_utf8(self.current_token.clone()).unwrap();
                    if self.cmd.is_empty() {
                        self.cmd = current_token_str;
                    } else {
                        self.args.push(current_token_str);
                    }
                    self.current_token.clear();
                }
                _ => {
                    if self.is_first_char && *ele == b' ' {
                        continue;
                    }
                    self.current_token.push(*ele);
                    self.is_first_char = false;
                }
            }
        }
        !(self.is_indisde_dapo || self.is_inside_apo)
    }

    fn print_leader(&self, symbol: char) {
        print!("{symbol} ");
        io::stdout().flush().unwrap();
    }
}
