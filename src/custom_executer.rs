use std::{env, fs::read_dir};

pub fn is_exec(cmd: &str) -> bool {
    locate(cmd).is_some()
}

pub fn locate(cmd: &str) -> Option<String> {
    let path = env::var("PATH").ok()?;
    path.split(':')
        .find(|path| {
            read_dir(path)
                .unwrap_or_else(|_| panic!("There was a problem reading {}", path))
                .find(|f| {
                    f.as_ref()
                        .map(|entry| entry.file_name().into_string().unwrap() == cmd)
                        .unwrap_or(false)
                })
                .is_some()
        })
        .map(|x| format!("{x}/{cmd}"))
}
