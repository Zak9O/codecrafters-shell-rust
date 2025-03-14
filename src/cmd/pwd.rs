use std::env::current_dir;

use super::{Cmd, StdOutput};

pub struct Pwd;

impl Cmd for Pwd {
    fn execute(&self, StdOutput(stdout,_): &mut StdOutput) -> () {
        let binding = current_dir().unwrap();
        let pwd = binding.to_str().unwrap();
        stdout.write(pwd.as_bytes()).unwrap();
    }
}

impl Pwd {
    pub fn new(_: &Vec<String>) -> Option<Self> {
        Some(Self)
    }
}
