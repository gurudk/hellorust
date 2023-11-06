use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let f = File::open("README.md").unwrap();
    let reader = BufReader::new(f);

    for line_ in reader.lines() {    // <1>
        let line = line_.unwrap();    // <2>
        println!("{} ({} bytes long)", line, line.len());
    }
}