#![allow(unused)]
use clap::Parser;
use std::fs::File;
use std::io::{BufReader, BufRead};
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let pathstr = args.path.as_os_str().to_str().unwrap();
    let f = File::open(&args.path)
        .with_context(|| format!("could not read file `{}`", &pathstr))?;
    let mut reader = BufReader::new(f);

    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line)?;
        if len == 0 { break; }
        if line.contains(&args.pattern) { println!("{}", line); }
    }

    Ok(())
}
