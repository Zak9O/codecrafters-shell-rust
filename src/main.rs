#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print_leader();
        let input = get_user_input();
        println!("{}: command not found", input.trim())
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
