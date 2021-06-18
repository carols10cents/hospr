use std::process;

fn main() {
    if let Err(e) = catr::run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
