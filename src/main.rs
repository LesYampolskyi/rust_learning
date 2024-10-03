use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("data/data.txt")
        .unwrap();

    if let Err(e) = writeln!(f, "A new line!") {
        eprintln!("Couldn't write to file: {}", e);
    }
}