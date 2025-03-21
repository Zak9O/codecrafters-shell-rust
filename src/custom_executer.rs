use std::{
    env,
    fs::{self, read_dir},
};

pub fn is_exec(cmd: &str) -> bool {
    locate(cmd).is_some()
}

pub fn locate(cmd: &str) -> Option<String> {
    let path = env::var("PATH").ok()?;
    path.split(':')
        .find(|path| {
            read_dir(path)
                .ok()
                .map(|entries| {
                    entries
                        .filter_map(Result::ok) // Skip any entries that have errors
                        .any(|entry| entry.file_name().into_string().unwrap_or_default() == cmd)
                })
                .unwrap_or(false) // If read_dir failed, return false for this path
        })
        .map(|x| format!("{x}/{cmd}"))
}

pub fn execs_in_path() -> Vec<String> {
    let path_var = env::var("PATH").ok(); // Get the PATH variable
    path_var
        .as_deref() 
        .unwrap_or("") 
        .split(":") 
        .filter_map(|p| fs::read_dir(p).ok()) 
        .flat_map(|entries| entries.filter_map(Result::ok)) 
        .map(|entry| entry.file_name().to_str().unwrap().to_owned())
        .collect()
}
