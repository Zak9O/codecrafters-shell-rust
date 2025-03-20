use std::{
    fs::{File, OpenOptions},
    io::{self, ErrorKind},
};

use codecrafters_shell::{
    cmd::{input_to_cmd, StdOutput},
    promt::{Command, OutputType, Promt, RedirectType, UserInput},
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
                println!("{:?}", user_input);
                match user_input {
                    UserInput::Command(Command(cmd, args)) => {
                        let cmd = input_to_cmd(&cmd, &args);
                        cmd.map(|x| x.execute(&mut std_output));
                    }
                    UserInput::Redirect(Command(cmd, args), redirect_type, file_name) => {
                        match redirect_type {
                            RedirectType::New(std_out) => {
                                let file_handle = Box::new(
                                    File::create_new(file_name).expect("Could not create file"),
                                );
                                match std_out {
                                    OutputType::Stdout => std_output.0 = file_handle,
                                    OutputType::Stderr => std_output.1 = file_handle,
                                }
                            }
                            RedirectType::Append(std_out) => {
                                println!("trying to append to file");
                                let file_handle = Box::new(
                                    OpenOptions::new()
                                        .write(true)
                                        .append(true)
                                        .create(true)
                                        .open(file_name)
                                        .unwrap(),
                                );
                                match std_out {
                                    OutputType::Stdout => std_output.0 = file_handle,
                                    OutputType::Stderr => std_output.1 = file_handle,
                                }
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
    }
}
