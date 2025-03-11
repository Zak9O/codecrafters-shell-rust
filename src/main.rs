use std::io::ErrorKind;

use codecrafters_shell::{cmd::input_to_cmd, parser::Parser};

fn main() {
    loop {
        let parser = Parser::new();
        let result = parser.get_user_input();
        match result {
            Err(ref e) if e.kind() == ErrorKind::InvalidData => {
                eprintln!("Error: Invalid UTF-8 data")
            }
            Ok((cmd, args)) => {
                let command = input_to_cmd(&cmd, &args);
                match command {
                    None => continue,
                    Some(cmd) => cmd.execute(),
                }
            }
            _ => (),
        }
    }
}
