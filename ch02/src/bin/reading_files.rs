use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("README.md").unwrap();
    let mut reader = BufReader::new(file);

    let mut line = String::new();

    loop {
        let len = reader.read_line(&mut line).unwrap();
        if len == 0 {
            break;
        }

        println!("{} ({} bytes long)", line, len);
        line.truncate(0);
    }
}
