use clap::App;

fn main() {
    let _matches = App::new("echor")
        .version("0.1.0")
        .author("Carol (Nichols || Goulding)")
        .about("Rust cat")
        .get_matches();
}
