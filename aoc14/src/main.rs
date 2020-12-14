use std::collections::HashMap;
use std::io::{self, Read};
use std::str::FromStr;
use std::time::Instant;

use anyhow::{anyhow, Error, Result};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Write { idx: usize, value: u64 },
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
    let mut addr_map: HashMap<usize, usize> = HashMap::new();

    let mut mem: Vec<u64> = vec![];
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

                if addr_map.contains_key(idx) {
                    mem[*addr_map.get(&idx).unwrap()] = masked_value;
                } else {
                    let mem_idx = mem.len();
                    mem.push(masked_value);
                    addr_map.insert(*idx, mem_idx);
                }
            }
        }
    }

    Ok(mem.iter().sum())
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
