use cd::Cd;
use custom::Custom;
use echo::Echo;
use exit::Exit;
use paste::paste;
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

        pub fn input_to_cmd(input: &str) -> Option<Box<dyn Cmd + '_>> {
            let mut iter = input.trim().split(' ');
            let cmd = iter.next().unwrap();
            let args: Vec<&str> = iter.collect();
            // Consider adding the below to macro as well
            let cmd: Box<dyn Cmd> = match cmd {
                "type" => Box::new(Type::new(args)?),
                $(
                     stringify!($name) => {
                        paste! {
                            Box::new([<$name:camel>]::new(args)?)
                        }
                     },
                )*
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
    };
}

pub mod custom;
// Type is reserved, so it has been hardcoded
pub mod r#type;
create_builtins!(echo, exit, pwd, cd);
