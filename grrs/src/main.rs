#![allow(unused)]
use clap::Parser;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let f = File::open(&args.path).expect(("Unable to open file"));
    let mut reader = BufReader::new(f);


    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line).expect("Unable to read line");
        if len == 0 {
            break;
        }
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

}
