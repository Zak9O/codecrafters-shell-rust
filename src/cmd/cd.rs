use std::env::set_current_dir;

use super::{Cmd, StdOutput};

pub struct Cd(String);

impl Cmd for Cd {
    fn execute(&self, _: &mut StdOutput) -> () {
        if self.0.trim().eq("~") {
            let path = dirs::home_dir().unwrap();
            let path = path.to_str().unwrap();
            self.set_dir(path);
        } else {
            self.set_dir(&self.0);
        }
    }
}

impl Cd {
    pub fn new(args: &Vec<String>) -> Option<Self> {
        if args.len() == 0 || args.len() > 1 {
            println!("cd expects one argument");
            return None;
        }
        let path = &args[0];
        Some(Cd(path.to_string()))
    }
    fn set_dir(&self, path: &str) -> () {
        match set_current_dir(path) {
            Ok(_) => (),
            Err(_) => println!("cd: {}: No such file or directory", self.0),
        }
    }
}
