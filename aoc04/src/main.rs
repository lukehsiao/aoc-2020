use std::io::{self, Read};
use std::str::FromStr;
use std::time::Instant;

use anyhow::{anyhow, Error, Result};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Default)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

impl FromStr for Passport {
    type Err = Error;

    fn from_str(s: &str) -> Result<Passport> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\b(\w+):(#?\w+)\b").unwrap();
        }

        let mut result: Passport = Passport::default();

        // Parse
        for cap in RE.captures_iter(s) {
            let key = &cap[1];
            let value = &cap[2];
            match key {
                "byr" => result.byr = String::from(value),
                "iyr" => result.iyr = String::from(value),
                "eyr" => result.eyr = String::from(value),
                "hgt" => result.hgt = String::from(value),
                "hcl" => result.hcl = String::from(value),
                "ecl" => result.ecl = String::from(value),
                "pid" => result.pid = String::from(value),
                "cid" => result.cid = Some(String::from(value)),
                _ => return Err(anyhow!("Unexpected key: {:#?}", cap)),
            }
        }

        // Validate
        if result.byr == String::default()
            || result.iyr == String::default()
            || result.eyr == String::default()
            || result.hgt == String::default()
            || result.hcl == String::default()
            || result.ecl == String::default()
            || result.pid == String::default()
        {
            Err(anyhow!("Invalid Passport: {:#?}", result))
        } else {
            Ok(result)
        }
    }
}

fn part1(input: &str) -> Result<()> {
    // Blank line separates entries
    let passports: Vec<Passport> = input.split("\n\n").filter_map(|l| l.parse().ok()).collect();

    println!("Part 1: {}", passports.len());
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    // Blank line separates entries
    let passports: Vec<Passport> = input.split("\n\n").filter_map(|l| l.parse().ok()).collect();

    // Extra validation
    let valid: Vec<&Passport> = passports
        .iter()
        .filter_map(|p| {
            let byr: u32 = p.byr.parse().unwrap();
            let iyr: u32 = p.iyr.parse().unwrap();
            let eyr: u32 = p.eyr.parse().unwrap();

            let mut result = Some(p);

            if byr < 1920 || byr > 2002 {
                result = None
            }

            if iyr < 2010 || iyr > 2020 {
                result = None
            }

            if eyr < 2020 || eyr > 2030 {
                result = None
            }

            match &p.hgt {
                s if s.ends_with("cm") => {
                    let cm: u32 = s[..s.len() - 2].parse().unwrap();
                    if cm < 150 || cm > 193 {
                        result = None
                    }
                }
                s if s.ends_with("in") => {
                    let inch: u32 = s[..s.len() - 2].parse().unwrap();
                    if inch < 59 || inch > 76 {
                        result = None
                    }
                }
                _ => result = None,
            }

            if p.hcl.chars().nth(0).unwrap() != '#' || p.hcl.len() != 7 {
                result = None
            } else {
                for c in p.hcl[1..].chars() {
                    match c {
                        '0'..='9' | 'a'..='f' => {}
                        _ => result = None,
                    }
                }
            }

            match p.ecl.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
                _ => result = None,
            }

            if p.pid.len() != 9 {
                result = None
            }

            result
        })
        .collect();

    println!("Part 2: {}", valid.len());
    Ok(())
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let now = Instant::now();
    part1(&input)?;
    println!("Part 1 took: {:#?}", now.elapsed());

    let now = Instant::now();
    part2(&input)?;
    println!("Part 2 took: {:#?}", now.elapsed());

    Ok(())
}
