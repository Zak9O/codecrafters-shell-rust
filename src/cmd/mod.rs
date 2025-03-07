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
        $(pub mod $name;)*

        // Type is reserved, so it has been hardcoded
        pub const BUILTINS: &[&str] = &["type", $(stringify!($name)),*];
    };
}

pub mod custom;
// Type is reserved, so it has been hardcoded
pub mod r#type;
create_builtins!(echo, exit, pwd);

pub fn input_to_cmd(input: &str) -> Option<Box<dyn Cmd + '_>> {
    let mut iter = input.trim().split(' ');
    let cmd = iter.next().unwrap();
    let args: Vec<&str> = iter.collect();
    let cmd: Box<dyn Cmd> = match cmd {
        "exit" => {
            println!("happened");
            Box::new(Exit::new(args)?)
        }
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
