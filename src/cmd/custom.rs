use std::process::Command;

use super::Cmd;

pub struct Custom<'a>(&'a str, Vec<&'a str>);

impl<'a> Cmd for Custom<'a> {
    fn execute(&self) -> () {
        let output = Command::new(&self.0)
            .args(&self.1)
            .output()
            .expect("Failed executing {exec_path}");
        println!("{}", String::from_utf8(output.stdout).unwrap())
    }
}

impl<'a> Custom<'a> {
    pub fn new(cmd: &'a str, args: Vec<&'a str>) -> Self {
        Self(cmd, args)
    }
}
