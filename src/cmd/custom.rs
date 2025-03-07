use std::process::Command;

use super::Cmd;

pub struct Custom<'a>(&'a str, Vec<&'a str>);

impl<'a> Cmd for Custom<'a> {
    fn execute(&self) -> () {
        Command::new(&self.0)
            .args(&self.1)
            .spawn()
            .expect("Failed executing {exec_path}");
    }
}

impl<'a> Custom<'a> {
    pub fn new(cmd: &'a str, args: Vec<&'a str>) -> Self {
        Self(cmd, args)
    }
}
