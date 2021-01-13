extern crate aoc_2020;

use std::env;
use std::str::Split;

#[derive(Clone, Default, Debug, PartialEq)]
struct ValidPassword {
    min: u8,
    max: u8,
    c: char,
    password: String,
}

impl ValidPassword {
    fn is_valid(&self) -> bool {
        let count = self.password.matches(self.c).count() as u8;
        self.min <= count && count <= self.max
    }

    fn get_char_at_idx(&self, idx: u8) -> Option<u8> {
        let idx = idx as usize - 1;
        if idx < self.password.as_bytes().len() {
            return Some(self.password.as_bytes()[idx]);
        }
        None
    }

    fn is_valid_pt2(&self) -> bool {
        let a = self.get_char_at_idx(self.min);
        let b = self.get_char_at_idx(self.max);

        (a == Some(self.c as u8) || b == Some(self.c as u8)) && a != b
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let version: u8 = args[2].parse().unwrap();

    if let Ok(lines) = aoc_2020::read_lines(filename) {
        let count = lines.fold(0, |acc, line| match version {
            2 => parse_line(line.unwrap()).is_valid_pt2() as u32 + acc,
            _ => parse_line(line.unwrap()).is_valid() as u32 + acc,
        });
        println!("total count of valid passwords: {}", count);
    } else {
        println!("Error opening file {}", filename);
    }
    Ok(())
}

fn parse_first_char(split: &mut Split<char>) -> char {
    split.next().unwrap().chars().next().unwrap()
}

fn parse_u8(split: &mut Split<char>) -> u8 {
    split.next().unwrap().parse().unwrap()
}

fn parse_password(split: &mut Split<char>) -> String {
    split.next().unwrap().to_string()
}

fn parse_min_max(split: &mut Split<char>) -> (u8, u8) {
    let mut minmax = split.next().unwrap().split('-');
    let min: u8 = parse_u8(&mut minmax);
    let max: u8 = parse_u8(&mut minmax);

    (min, max)
}

fn parse_line(i: String) -> ValidPassword {
    let mut split = i.split(' ');

    let (min, max) = parse_min_max(&mut split);

    ValidPassword {
        min,
        max,
        c: parse_first_char(&mut split),
        password: parse_password(&mut split),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /* test cases
     *
     * 2-4 c: cbccc
     * 4-5 q: cwmhd
     */

    #[test]
    fn test_valid_password() {
        let pw = ValidPassword {
            min: 2,
            max: 4,
            c: 'c',
            password: "cbccc".to_string(),
        };
        assert!(pw.is_valid());
    }

    #[test]
    fn test_invalid_password() {
        let pw = ValidPassword {
            min: 4,
            max: 5,
            c: 'q',
            password: "cwmhd".to_string(),
        };
        assert!(!pw.is_valid());
    }

    #[test]
    fn test_is_valid_pt2() {
        let pw1 = ValidPassword {
            min: 1,
            max: 3,
            c: 'a',
            password: "abcde".to_string(),
        };

        let pw2 = ValidPassword {
            min: 1,
            max: 3,
            c: 'b',
            password: "cdefg".to_string(),
        };

        let pw3 = ValidPassword {
            min: 2,
            max: 9,
            c: 'c',
            password: "ccccccccc".to_string(),
        };
        assert!(pw1.is_valid_pt2());
        assert!(!pw2.is_valid_pt2());
        assert!(!pw3.is_valid_pt2());
    }

    #[test]
    fn test_parse_line() {
        let input = "2-4 c: cbccc".to_string();
        let pw = ValidPassword {
            min: 2,
            max: 4,
            c: 'c',
            password: "cbccc".to_string(),
        };
        assert_eq!(parse_line(input), pw);
    }
}
