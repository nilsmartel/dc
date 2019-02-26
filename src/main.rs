mod io;
mod state_machine;

fn main() {
    let code = if let Some(filename) = std::env::args().nth(1) {
        io::read_file(&filename).expect("Failed to read file")
    } else {
        io::read_stdin()
    };

    let mut state = state_machine::State::from_sequence(code.lines());

    loop {
        match state.next_operation() {
            Some(value) => println!("{}", value),
            None => break,
        }
    }
}
