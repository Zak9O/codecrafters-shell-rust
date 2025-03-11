use crate::custom_executer::locate;

use super::{Cmd, BUILTINS};
use CmdType::*;

enum CmdType {
    Invalid,
    Bultin,
    Executeable(String),
}

pub struct Type(CmdType, String);

impl Cmd for Type {
    fn execute(&self) -> () {
        match self.0 {
            Invalid => println!("{}: not found", self.1),
            Bultin => println!("{} is a shell builtin", self.1),
            Executeable(ref full_path) => println!("{} is {}", self.1, full_path),
        }
    }
}

impl Type {
    pub fn new(args: &Vec<String>) -> Option<Self> {
        if args.is_empty() || args.len() > 1 {
            println!("type expects one argument");
            return None;
        }

        let cmd = args.first().unwrap().to_string();
        let cmd = if BUILTINS.contains(&&cmd[..]) {
            Self(Bultin, cmd)
        } else {
            match locate(&cmd) {
                None => Self(Invalid, cmd),
                Some(full_path) => Self(Executeable(full_path), cmd),
            }
        };
        Some(cmd)
    }
}
