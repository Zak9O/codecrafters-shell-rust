use std::io::{Error, ErrorKind};

use super::{Command, RedirectType, UserInput};



pub struct Parser {
    user_input: UserInput,
    current_token: Vec<u8>,
    is_inside_apo_block: bool,
    is_indisde_dapo_block: bool,
    is_first_char_in_token: bool,
    is_escaped: bool,
    skip: bool,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            user_input: UserInput::Command(Command::new()),
            current_token: Vec::new(),
            is_inside_apo_block: false,
            is_indisde_dapo_block: false,
            is_first_char_in_token: true,
            is_escaped: false,
            skip: false,
        }
    }

    fn is_in_block(&self) -> bool {
        self.is_inside_apo_block || self.is_indisde_dapo_block
    }

    pub fn parse(&mut self, input: &str) -> Result<UserInput, Error> {
        let bytes = input.as_bytes();
        for i in 0..bytes.len() {
            if self.skip {
                self.skip = false;
                continue;
            }
            let ele = bytes.get(i).unwrap();
            let next_element = bytes.get(i);
            match ele {
                _ if self.is_escaped => {
                    let is_ele_special_char = [b'\"', b'\\', b'$'].contains(ele);
                    if (self.is_indisde_dapo_block && !is_ele_special_char)
                        || self.is_inside_apo_block
                    {
                        self.current_token.push(b'\\');
                    }
                    self.current_token.push(*ele);
                    self.is_escaped = false;
                }
                b'\\' => self.is_escaped = true,
                b'\'' if !self.is_indisde_dapo_block => {
                    self.is_inside_apo_block = !self.is_inside_apo_block
                }
                b'\"' if !self.is_inside_apo_block => {
                    self.is_indisde_dapo_block = !self.is_indisde_dapo_block
                }
                b' ' | b'\n' if !self.is_in_block() && !self.is_first_char_in_token => {
                    self.is_first_char_in_token = true;
                    let current_token_str = String::from_utf8(self.current_token.clone()).unwrap();
                    match &mut self.user_input {
                        UserInput::Command(cmd) => {
                            if cmd.0.is_empty() {
                                cmd.0 = current_token_str;
                            } else {
                                cmd.1.push(current_token_str);
                            }
                        }
                        UserInput::Redirect(_, _, file_name) => {
                            *file_name = current_token_str;
                        }
                    };
                    self.current_token.clear();
                }
                b'1' if next_element.is_some_and(|x| *x == b'>') && !self.is_in_block() => {
                    self.is(*ele)?
                }
                b'>' if !self.is_in_block() => self.is(*ele)?,
                _ => {
                    if self.is_first_char_in_token && *ele == b' ' {
                        continue;
                    }
                    self.current_token.push(*ele);
                    self.is_first_char_in_token = false;
                }
            }
        }
        if self.is_in_block() {
            Err(Error::new(ErrorKind::InvalidInput, "Input was incomplete"))
        } else {
            Ok(self.user_input.clone())
        }
    }

    fn is(&mut self, ele: u8) -> Result<(), Error> {
        self.skip = true;
        let cmd = match &self.user_input {
            UserInput::Redirect(_, _, _) => {
                return Err(Error::new(ErrorKind::InvalidData, "Input cannot be passed"))
            }
            UserInput::Command(cmd) => cmd,
        };
        let redirect_type = match ele {
            b'1' | b'>' => RedirectType::Stdin,
            _ => unreachable!(),
        };
        self.user_input = UserInput::Redirect(cmd.clone(), redirect_type, String::new());
        Ok(())
    }
}
