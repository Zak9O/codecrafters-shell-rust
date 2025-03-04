#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

enum Cmd<'a> {
    Exit(i32),
    Echo(Vec<&'a str>),
    Type(&'a str),
    Invalid,
}

fn main() {
    loop {
        print_leader();
        let input = get_user_input();
        let input = handle_user_input(&input);
        match input {
            Cmd::Invalid => continue,
            Cmd::Echo(args) => {
                println!("{}", args.join(" "))
            }
            Cmd::Type(cmd) => {
                if ["type", "exit", "echo"].contains(&cmd) {
                    println!("{cmd} is a shell builtin");
                } else {
                    println!("{}: not found", cmd);
                }
            }
            Cmd::Exit(x) => exit(x),
        }
    }
}

fn handle_user_input<'a>(input: &'a str) -> Cmd<'a> {
    let mut iter = input.trim().split(' ');
    let cmd = iter.next().unwrap();
    let args: Vec<&str> = iter.collect();
    match cmd {
        "exit" => {
            if args.len() == 0 || args.len() > 1 {
                println!("exit expects one integer argument");
                return Cmd::Invalid;
            }
            let exit_code: i32 = match args[0].parse() {
                Err(_) => {
                    println!("{} was expected to be an integer", args[0]);
                    return Cmd::Invalid;
                }
                Ok(x) => x,
            };

            Cmd::Exit(exit_code)
        }
        "echo" => Cmd::Echo(args),
        "type" => {
            if args.len() == 0 || args.len() > 1 {
                println!("type expects one argument");
                return Cmd::Invalid;
            }
            Cmd::Type(args[0])
        }
        cmd => {
            println!("{}: command not found", cmd);
            Cmd::Invalid
        }
    }
}

fn get_user_input() -> String {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    input
}

fn print_leader() {
    print!("$ ");
    io::stdout().flush().unwrap();
}
