#[macro_use]
extern crate lazy_static;

use core::str::FromStr;
use std::collections::HashSet;
use std::env;

lazy_static! {
    static ref VALID_ECL: HashSet<&'static str> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .cloned()
        .collect();
}

#[derive(Debug, Default)]
struct Passport {
    pid: Option<String>,
    cid: Option<String>,
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        !(self.pid.is_none()
            || self.byr.is_none()
            || self.iyr.is_none()
            || self.eyr.is_none()
            || self.hgt.is_none()
            || self.hcl.is_none()
            || self.ecl.is_none())
    }

    fn parse_pid(&mut self, value: &str) {
        self.pid = match value.parse::<String>() {
            Ok(v) if v.chars().all(char::is_numeric) && v.len() == 9 => Some(v),
            _ => None,
        }
    }

    fn parse_cid(&mut self, value: &str) {
        self.cid = match value.parse::<String>() {
            Ok(v) => Some(v),
            _ => None,
        }
    }

    fn parse_byr(&mut self, value: &str) {
        self.byr = match value.parse() {
            Ok(v) if v >= 1920 && v <= 2020 => Some(v),
            _ => None,
        }
    }

    fn parse_iyr(&mut self, value: &str) {
        self.iyr = match value.parse() {
            Ok(v) if v >= 2010 && v <= 2020 => Some(v),
            _ => None,
        }
    }

    fn parse_eyr(&mut self, value: &str) {
        self.eyr = match value.parse() {
            Ok(v) if v >= 2020 && v <= 2030 => Some(v),
            _ => None,
        }
    }

    fn parse_hgt(&mut self, value: &str) {
        let raw_val = value.as_bytes();
        if raw_val.len() < 4 {
            self.hgt = None;
            return;
        }

        if raw_val[3] == b'c' {
            // val is in cm
            let hgt_cm: u16 = std::str::from_utf8(&raw_val[..3]).unwrap().parse().unwrap();
            if hgt_cm > 150 && hgt_cm <= 193 {
                self.hgt = Some(value.parse().unwrap());
            }
        } else if raw_val[2] == b'i' {
            // val is in in
            let hgt_in: u16 = std::str::from_utf8(&raw_val[..2]).unwrap().parse().unwrap();
            if hgt_in > 59 && hgt_in <= 76 {
                self.hgt = Some(value.parse().unwrap());
            }
        }

    }

    fn parse_hcl(&mut self, value: &str) {
        if value.as_bytes()[0] == b'#' {
            self.hcl = Some(value.parse().unwrap());
        }
    }

    fn parse_ecl(&mut self, value: &str) {
        self.ecl = match value.parse::<String>() {
            Ok(v) => {
                if VALID_ECL.contains(v.as_str()) {
                    Some(v.to_string())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl FromStr for Passport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut passport: Passport = Default::default();

        let input = s.split(' ');
        for kv in input {
            let mut kv_parsed = kv.split(':');
            match (kv_parsed.next(), kv_parsed.next()) {
                (Some("pid"), Some(value)) => passport.parse_pid(value),
                (Some("cid"), Some(value)) => passport.parse_cid(value),
                (Some("byr"), Some(value)) => passport.parse_byr(value),
                (Some("iyr"), Some(value)) => passport.parse_iyr(value),
                (Some("eyr"), Some(value)) => passport.parse_eyr(value),
                (Some("hgt"), Some(value)) => passport.parse_hgt(value),
                (Some("hcl"), Some(value)) => passport.parse_hcl(value),
                (Some("ecl"), Some(value)) => passport.parse_ecl(value),
                _ => {}
            }
        }

        Ok(passport)
    }
}

fn parse_args() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    filename.to_string()
}

fn main() -> std::io::Result<()> {
    let filename = parse_args();

    let mut count: u16 = 0;
    let mut total: u16 = 0;
    let mut parse_buffer = String::new();

    if let Ok(lines) = aoc_2020::read_lines(filename) {
        for line in lines {
            match line {
                Ok(line) if &line != "" => {
                    parse_buffer.push_str(&line);
                    parse_buffer.push(' ');
                }
                _ => {
                    if parse_buffer.parse::<Passport>().unwrap().is_valid() {
                        count += 1;
                    };
                    total += 1;
                    parse_buffer = String::new();
                }
            }
        }
        if parse_buffer.parse::<Passport>().unwrap().is_valid() {
            count += 1;
        };
        total += 1;
    }

    println!("Total passport counts = {}", total);
    println!("Valid passport counts = {}", count);
    Ok(())
}
