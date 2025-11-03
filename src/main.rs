use clap::Parser;
use colored::*;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

#[derive(Parser)]
#[command(name = "loggrep")]
#[command(about = "A lightweight log filtering tool made in Rust")]
struct Args {
    #[arg(help = "Path to log file")]
    file: String,

    #[arg(help = "Keyword to search for")]
    keyword: String,

    #[arg(short, long, help = "Show line numbers")]
    lines: bool,
}

// Main function
fn main() {
    let args = Args::parse();

    if let Ok(lines) = read_lines(&args.file) {
        for (index, line) in lines.map_while(Result::ok).enumerate() {
            if line.contains(&args.keyword) {
                let highlighted =
                    line.replace(&args.keyword, &args.keyword.red().bold().to_string());
                if args.lines {
                    println!("{}: {}", index + 1, highlighted);
                } else {
                    println!("{}", highlighted);
                }
            }
        }
    }
}

// Function to read lines from a file and return an iterator
fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}
