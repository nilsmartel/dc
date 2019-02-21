mod io;

fn main() {
    let code = if let Some(filename) = std::env::args().nth(1) {
        io::read_file(&filename).expect("Failed to read file");
    } else {
        io::read_stdin();
    };
}
