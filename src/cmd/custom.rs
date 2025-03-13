use std::process::Command;

use super::Cmd;

pub struct Custom<'a>(&'a str, &'a Vec<String>);

impl<'a> Cmd for Custom<'a> {
    fn execute(&self) -> Option<String> {
        let output = Command::new(&self.0)
            .args(self.1)
            .output()
            .expect("Failed executing {exec_path}");
        Some( String::from_utf8(output.stdout).unwrap())
    }
}

impl<'a> Custom<'a> {
    pub fn new(cmd: &'a str, args: &'a Vec<String>) -> Self {
        Self(cmd, args)
    }
}
