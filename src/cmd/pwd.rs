use std::env::current_dir;

use super::Cmd;

pub struct Pwd;

impl Cmd for Pwd {
    fn execute(&self) -> Option<String> {
        let binding = current_dir().unwrap();
        let pwd = binding.to_str().unwrap();
        Some(pwd.to_string())
    }
}

impl Pwd {
    pub fn new(_: &Vec<String>) -> Option<Self> {
        Some(Self)
    }
}
