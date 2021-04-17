use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::new("pattern")
                .about("The pattern to search for")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("input")
                .about("File to search")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let input = args.value_of("input").unwrap();
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        if re.find(&line).is_some() {
            println!("{:?}", line);
        }
    }
}
