use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Password {
    counter: HashMap<char, u32>,
    target: char,
    num_1: u32,
    num_2: u32,
    password: String,
}

impl FromStr for Password {
    type Err = Error;

    fn from_str(s: &str) -> Result<Password> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                (?P<num_1>[0-9]+)-(?P<num_2>[0-9]+)\s+
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

        Ok(Password {
            counter,
            target: caps["target"].parse()?,
            num_1: caps["num_1"].parse()?,
            num_2: caps["num_2"].parse()?,
            password,
        })
    }
}

// How many passwords are valid?
fn part1(input: &str) -> Result<()> {
    let passwords: Vec<Password> = input.lines().filter_map(|l| l.parse().ok()).collect();

    let valid_passwords: Vec<&Password> = passwords
        .iter()
        .filter_map(|pass| match pass.counter.get(&pass.target) {
            Some(freq) => {
                if freq < &pass.num_1 || freq > &pass.num_2 {
                    None
                } else {
                    Some(pass)
                }
            }
            None => None,
        })
        .collect();

    println!("Part 1: {}", valid_passwords.len());

    Ok(())
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;

    Ok(())
}
