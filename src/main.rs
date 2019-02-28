mod io;
mod state_machine;

use state_machine::State;

fn main() {
    let code = get_code();

    let mut state = State::from_sequence(code.lines());

    loop {
        match state.next_operation() {
            Some(value) => println!("{:?}", value),
            None => break,
        }
    }
}

fn get_code() -> String {
    if let Some(filename) = std::env::args().nth(1) {
        io::read_file(&filename).expect(&format!("Failed to read file {}", filename))
    } else {
        io::read_stdin()
    }
}
