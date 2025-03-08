use std::process::exit;

use super::Cmd;

pub struct Exit(i32);
impl Cmd for Exit {
    fn execute(&self) -> () {
        exit(self.0)
    }
}
impl Exit {
    pub fn new_empty() -> Self {
        Self(0)
    }

    pub fn new(args: Vec<&str>) -> Option<Self> {
        if args.len() == 0 || args.len() > 1 {
            println!("exit expects one integer argument");
            return None;
        }
        let exit_code: i32 = match args[0].parse() {
            Err(_) => {
                println!("{} was expected to be an integer", args[0]);
                return None;
            }
            Ok(x) => x,
        };
        Some(Self(exit_code))
    }
}
