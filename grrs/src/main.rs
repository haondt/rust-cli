#![allow(unused)]
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("pattern: {}", args.pattern);
    println!("path: {}", args.path.into_os_string().into_string().unwrap());
}
