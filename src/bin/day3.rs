extern crate aoc_2020;

use std::env;

fn parse_args() -> (String, usize, usize) {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut right: usize = 3;
    let mut down: usize = 1;

    if args.len() > 2 {
        right = args[2].parse().unwrap();
    }
    if args.len() > 3 {
        down = args[3].parse().unwrap();
    }

    (filename.to_string(), right, down)
}

fn main() -> std::io::Result<()> {
    let (filename, right, down) = parse_args();

    if let Ok(lines) = aoc_2020::read_lines(filename) {
        // so we can see how long each line is...
        let mut lines = lines.peekable();
        let mut idx = 0;
        let mut count = 0;
        let line_len = lines.peek().unwrap().as_ref().unwrap().len();

        for line in lines.step_by(down) {
            if line.unwrap().as_bytes()[idx] == b'#' {
                count += 1;
            }

            idx = (idx + right) % line_len;
        }

        println!("Trees passed = {}", count);
    }
    Ok(())
}
