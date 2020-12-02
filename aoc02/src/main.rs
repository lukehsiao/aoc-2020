use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct PassEntry {
    counter: HashMap<char, u32>,
    target: char,
    min_freq: u32,
    max_freq: u32,
    password: String,
}

impl FromStr for PassEntry {
    type Err = Error;

    fn from_str(s: &str) -> Result<PassEntry> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                (?P<min_freq>[0-9]+)-(?P<max_freq>[0-9]+)\s+
                (?P<target>\w):\s+
                (?P<password>\w+)
                "
            )
            .unwrap();
        }

        let caps = match RE.captures(s) {
            None => return Err(anyhow!("No match: {}", s)),
            Some(c) => c,
        };

        // Count letter frequencies
        let password: String = caps["password"].parse()?;
        let mut counter: HashMap<char, u32> = HashMap::new();
        for c in password.chars() {
            if counter.contains_key(&c) {
                if let Some(count) = counter.get_mut(&c) {
                    *count += 1;
                }
            } else {
                counter.insert(c, 1);
            }
        }

        let result = PassEntry {
            counter,
            target: caps["target"].parse()?,
            min_freq: caps["min_freq"].parse()?,
            max_freq: caps["max_freq"].parse()?,
            password,
        };

        // Validate constraint
        match result.counter.get(&result.target) {
            Some(freq) => {
                if freq < &result.min_freq || freq > &result.max_freq {
                    Err(anyhow!("Invalid password (invalid freq): {:#?}", result))
                } else {
                    Ok(result)
                }
            }
            None => Err(anyhow!("Invalid password (missing target): {:#?}", result)),
        }
    }
}

// How many passwords are valid?
fn part1(input: &str) -> Result<()> {
    let mut passwords: Vec<PassEntry> = vec![];
    for line in input.lines() {
        match line.parse() {
            Ok(password) => passwords.push(password),
            Err(e) => eprintln!("{}", e),
        }
    }

    println!("Part 1: {}", passwords.len());

    Ok(())
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;

    Ok(())
}
