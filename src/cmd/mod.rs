use custom::Custom;
use echo::Echo;
use exit::Exit;
use pwd::Pwd;
use r#type::Type;

use crate::custom_executer::is_exec;

pub trait Cmd {
    fn execute(&self) -> ();
}

macro_rules! create_builtins {
    ($($name:ident),*) => {
        // Define the modules
        $(pub mod $name;)*

        // Define the BUILTINS constant as an array of strings
        pub const BUILTINS: &[&str] = &[$(stringify!($name)),*];
    };
}

pub mod custom;
create_builtins!(echo, exit, pwd, r#type);

pub fn input_to_cmd(input: &str) -> Option<Box<dyn Cmd + '_>> {
    let mut iter = input.trim().split(' ');
    let cmd = iter.next().unwrap();
    let args: Vec<&str> = iter.collect();
    let cmd: Box<dyn Cmd> = match cmd {
        "exit" => {

            println!("happened");
            Box::new(Exit::new(args)?)},
        "echo" => Box::new(Echo::new(args)),
        "type" => Box::new(Type::new(args)?),
        "pwd" => Box::new(Pwd::new()),
        cmd => {
            if is_exec(cmd) {
                Box::new(Custom::new(cmd, args))
            } else {
                println!("{}: command not found", cmd);
                return None;
            }
        }
    };
    Some(cmd)
}
