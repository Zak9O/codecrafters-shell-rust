use std::process::Command;

use super::{Cmd, StdOutput};
pub struct Custom<'a>(&'a str, &'a Vec<String>);

impl<'a> Cmd for Custom<'a> {
    fn execute(&self, StdOutput(stdout, stderr): &mut StdOutput) -> () {
        let output = Command::new(&self.0)
            .args(self.1)
            .output()
            .expect("Failed executing {exec_path}");
        stdout.write(&output.stdout).unwrap();
        stderr.write(&output.stderr).unwrap();
    }
}

impl<'a> Custom<'a> {
    pub fn new(cmd: &'a str, args: &'a Vec<String>) -> Self {
        Self(cmd, args)
    }
}
