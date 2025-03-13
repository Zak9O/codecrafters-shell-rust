use std::{fs::File, io::{ErrorKind, Write}, process::Output};

use codecrafters_shell::{
    cmd::input_to_cmd,
    promt::{Command, Promt, UserInput},
};

fn main() {
    loop {
        let promt = Promt::new();
        let result = promt.prompt_user();
        match result {
            Err(ref e) if e.kind() == ErrorKind::InvalidData => {
                eprintln!("Error: Invalid UTF-8 data")
            }
            Ok(user_input) => match user_input {
                UserInput::Command(Command(cmd, args)) => {
                    let cmd = input_to_cmd(&cmd, &args);
                    cmd.map(|cmd| cmd.execute().map(|output| println!("{output}")));
                }
                UserInput::Redirect(Command(cmd, args), redirect_type, file_name) => {
                    let cmd = input_to_cmd(&cmd, &args);
                    let output = match cmd.map(|cmd| cmd.execute()) {
                        Some(Some(output)) => output,
                        _ => String::new(),
                    };
                    let mut file = File::create_new(file_name).expect("Could not create file");
                    file.write(output.trim().as_bytes()).expect("Could not write to file");
                }
            },
            _ => (),
        }
    }
}
