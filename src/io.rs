use std::fs::File;
use std::io::{stdin, Read};

pub fn read_stdin() -> String {
    let mut buffer = String::new();
    let _ = stdin().read_to_string(&mut buffer);

    buffer
}

pub fn read_file(filename: &str) -> Option<String> {
    if let Ok(mut file) = File::open(filename) {
        let mut buffer = String::new();

        let _ = file.read_to_string(&mut buffer);

        return Some(buffer);
    }

    None
}
