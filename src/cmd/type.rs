use crate::custom_executer::locate;

use super::{Cmd, StdOutput, BUILTINS};
use CmdType::*;

enum CmdType {
    Invalid,
    Bultin,
    Executeable(String),
}

pub struct Type(CmdType, String);

impl Cmd for Type {
    fn execute(&self, StdOutput(stdout, _): &mut StdOutput) -> () {
        match self.0 {
            Invalid => stdout.write(self.1.as_bytes()).unwrap(),
            Bultin => stdout.write(self.1.as_bytes()).unwrap(),
            Executeable(ref full_path) => stdout
                .write(format!("{} is {}", self.1, full_path).as_bytes())
                .unwrap(),
        };
        stdout.write(b"\n").unwrap();
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
