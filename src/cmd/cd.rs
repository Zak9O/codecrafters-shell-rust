use std::env::set_current_dir;

use super::Cmd;

pub struct Cd(String);

impl Cmd for Cd {
    fn execute(&self) -> () {
        match set_current_dir(&self.0) {
            Ok(_) => (),
            Err(_) => println!("cd: {}: No such file or directory", self.0)
        }
    }
}

impl Cd {
    pub fn new(args: Vec<&str>) -> Option<Self> {
        if args.len() == 0 || args.len() > 1 {
            println!("cd expects one argument");
            return None;
        }
        let path = args[0];
        Some(Cd(path.to_string()))
    }
}

