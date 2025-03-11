use codecrafters_shell::cmd::input_to_cmd;
use core::str;
use std::io::{self, Write};

fn main() {
    loop {
        let input = get_user_input();
        if input.trim().is_empty() {
            continue;
        }
        let (cmd, args) = handle_input(&input);

        let command = input_to_cmd(&cmd, &args);
        match command {
            None => continue,
            Some(cmd) => cmd.execute(),
        }
    }
}
fn handle_input(input: &str) -> (String, Vec<String>) {
    let (cmd, args, _, _) = input.as_bytes().iter().fold(
        (String::new(), Vec::new(), Vec::new(), false),
        |(cmd, mut args, mut current_token, is_inside_apo), ele| match ele {
            b'\'' => (cmd, args, current_token, !is_inside_apo),
            b' ' | b'\n' if !is_inside_apo => {
                let current_token = String::from_utf8(current_token).unwrap();
                if cmd.is_empty() {
                    (current_token, args, Vec::new(), is_inside_apo)
                } else {
                    args.push(current_token);
                    (cmd, args, Vec::new(), is_inside_apo)
                }
            }
            _ => {
                current_token.push(*ele);
                (cmd, args, current_token, is_inside_apo)
            }
        },
    );
    (cmd, args)
}

fn get_user_input() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    print_leader('$');
    loop {
        stdin.read_line(&mut input).unwrap();
        if input.as_bytes().iter().filter(|&&x| x == b'\'').count() % 2 == 1 {
            print_leader('-');
        } else {
            break;
        }
    }
    input
}

fn print_leader(symbol:char) {
    print!("{symbol} ");
    io::stdout().flush().unwrap();
}
