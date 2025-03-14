use std::{
    fs::File,
    io::{self, ErrorKind, Write},
};

use codecrafters_shell::{
    cmd::{input_to_cmd, StdOutput},
    promt::{Command, Promt, RedirectType, UserInput},
};

fn main() {
    loop {
        let promt = Promt::new();
        let result = promt.prompt_user();
        let mut std_output = StdOutput(Box::new(io::stdout()), Box::new(io::stderr()));
        match result {
            Err(ref e) if e.kind() == ErrorKind::InvalidData => {
                eprintln!("Error: Invalid UTF-8 data")
            }
            Ok(user_input) => {
                match user_input {
                    UserInput::Command(Command(cmd, args)) => {
                        let cmd = input_to_cmd(&cmd, &args);
                        cmd.map(|x| x.execute(&mut std_output));
                    }
                    UserInput::Redirect(Command(cmd, args), redirect_type, file_name) => {
                        let file_handle =
                            Box::new(File::create_new(file_name).expect("Could not create file"));
                        match redirect_type {
                            RedirectType::Stdin => {
                                std_output.0 = file_handle;
                            }
                            RedirectType::Stderr => {
                                std_output.1 = file_handle;
                            }
                        }

                        let cmd = input_to_cmd(&cmd, &args);
                        cmd.map(|x| x.execute(&mut std_output));
                    }
                };
                std_output.flush();
            }
            _ => (),
        }
        println!("");
    }
}
