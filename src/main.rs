use codecrafters_shell::cmd::input_to_cmd;
use std::io::{self, Write};

fn main() {
    loop {
        print_leader();
        let input = get_user_input();
        if input.trim().is_empty() {
            continue;
        }
        let command = input_to_cmd(&input);
        match command {
            None => continue,
            Some(cmd) => cmd.execute(),
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
