use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("README.md").unwrap();
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}
