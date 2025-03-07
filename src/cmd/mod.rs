use custom::Custom;
use echo::Echo;
use exit::Exit;
use types::Types;

use crate::custom_executer::{is_exec, locate};

pub trait Cmd {
    fn execute(&self) -> ();
}

pub fn input_to_cmd(input: &str) -> Option<Box<dyn Cmd + '_>> {
    let mut iter = input.trim().split(' ');
    let cmd = iter.next().unwrap();
    let args: Vec<&str> = iter.collect();
    let cmd: Box<dyn Cmd> = match cmd {
        "exit" => Box::new(Exit::new(args)?),
        "echo" => Box::new(Echo::new(args)),
        "type" => Box::new(Types::new(args)?),
        cmd => {
            if is_exec(cmd) {
                Box::new(Custom::new(cmd, args))
            } else {
                println!("{} is not a command", cmd);
                return None;
            }
        }
    };
    Some(cmd)
}

pub mod custom;
pub mod echo;
pub mod exit;
pub mod types;
