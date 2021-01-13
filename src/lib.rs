use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Wrap output in a result to allow for matching on errors,
// returns an Iterator of lines from the file
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
