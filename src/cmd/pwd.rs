use std::env::current_dir;

use super::Cmd;

pub struct Pwd;

impl Cmd for Pwd {
    fn execute(&self) -> () {
        let binding = current_dir().unwrap();
        let pwd = binding.to_str().unwrap();
        println!("{pwd}");
    }
}

impl Pwd {
    pub fn new(_: &Vec<String>) -> Option<Self> {
        Some(Self)
    }
}
