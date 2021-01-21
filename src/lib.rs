use std::env;
use std::fs::File;
use std::io::Split;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub type InputResult = io::Result<InputLines>;
pub type InputLines = io::Lines<io::BufReader<File>>;

// Simple helper for single arg CLI
pub fn parse_simple_args() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    filename.to_string()
}

// Wrap output in a result to allow for matching on errors,
// returns an Iterator of lines from the file
pub fn read_lines<P>(filename: P) -> InputResult
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_split<P>(filename: P, c: u8) -> io::Result<Split<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    Ok(reader.split(c))
}
