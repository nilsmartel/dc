use std::fs::File;
use std::io::{stdin, Read};

pub fn read_stdin() -> Option<String> {
    let mut buffer = String::new();
    match stdin().read_to_string(&mut buffer) {
        Ok(_) => Some(buffer),
        _ => None,
    }
}

pub fn read_file(filename: &str) -> Option<String> {
    if let Ok(mut file) = File::open(filename) {
        let mut buffer = String::new();

        let _ = file.read_to_string(&mut buffer);

        return Some(buffer);
    }

    None
}
