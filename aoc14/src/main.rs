use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;
use std::time::Instant;

use anyhow::{anyhow, Error, Result};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Write { idx: u64, value: u64 },
    Mask { and: u64, or: u64 },
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Instruction> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^mem\[(?P<idx>\d+)\] = (?P<value>\d+)").unwrap();
        }

        if let Some(caps) = RE.captures(s) {
            let idx = caps["idx"].parse()?;
            let value = caps["value"].parse()?;

            Ok(Instruction::Write { idx, value })
        } else if s.starts_with("mask = ") {
            // OR-ing with a 1 spot forces 1, 0 does nothing
            // AND-ing with a 0 spot forces 0, 1 does nothing
            let orig = &s[7..];

            let and = u64::from_str_radix(&orig.replace("X", "1"), 2)?;
            let or = u64::from_str_radix(&orig.replace("X", "0"), 2)?;

            Ok(Instruction::Mask { and, or })
        } else {
            Err(anyhow!("Invalid input: \"{}\"", s))
        }
    }
}

fn part1(input: &Vec<Instruction>) -> Result<u64> {
    // Don't actually care about all memory locations. Just track the mapping we care about.
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut current_mask = Instruction::Mask { and: 0, or: 0 };

    for inst in input {
        match inst {
            Instruction::Mask { and: _, or: _ } => current_mask = *inst,
            Instruction::Write { idx, value } => {
                let mut masked_value: u64 = *value;

                if let Instruction::Mask { and, or } = current_mask {
                    masked_value &= and;
                    masked_value |= or;
                }

                mem.insert(*idx, masked_value);
            }
        }
    }

    Ok(mem.values().sum())
}

fn part2(input: &Vec<Instruction>) -> Result<u64> {
    // Don't actually care about all memory locations. Just track the mapping we care about.
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut force_1 = 0;
    let mut masks: Vec<u64> = vec![0];

    for inst in input {
        match inst {
            Instruction::Mask { and, or } => {
                force_1 = *or;
                masks.truncate(1);

                // First, get back all the X's which we discarded in pre-processing.
                let mut x = or ^ and;

                // Generate all the possible addr
                while x != 0 {
                    let lowest_set_bit = x ^ (x & (x - 1));
                    for i in 0..=masks.len() {
                        masks.push(masks[i] ^ lowest_set_bit);
                    }

                    // Unset the lowest bit of X
                    x &= x - 1;
                }
            }
            Instruction::Write { idx, value } => {
                for mask in &masks {
                    mem.insert((idx | force_1) ^ mask, *value);
                }
            }
        }
    }

    Ok(mem.values().sum())
}

fn main() -> Result<()> {
    // Process input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let instructions: Vec<Instruction> = input.lines().filter_map(|l| l.parse().ok()).collect();

    let now = Instant::now();
    match part1(&instructions) {
        Ok(v) => println!("Part 1: {}, took {:#?}", v, now.elapsed()),
        Err(e) => eprintln!("{}", e),
    };

    let now = Instant::now();
    match part2(&instructions) {
        Ok(v) => println!("Part 1: {}, took {:#?}", v, now.elapsed()),
        Err(e) => eprintln!("{}", e),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() -> Result<()> {
        Ok(())
    }
}
