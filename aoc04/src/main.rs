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

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let now = Instant::now();
    part1(&input)?;
    println!("Part 1 took: {:#?}", now.elapsed());

    Ok(())
}
