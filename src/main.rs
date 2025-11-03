use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: loggrep <logfile> <search_term>");
        std::process::exit(1);
    }

    let path = &args[1];
    let search_term = &args[2];
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            if line.contains(search_term) {
                println!("{}", line);
            }
        }
    }
}

// Function to read lines from a file and return an iterator
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

