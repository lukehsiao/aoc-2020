use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::{self, Read};
use std::time::Instant;

use anyhow::Result;

fn part1(input: &Vec<usize>, turns: usize) -> Result<usize> {
    // Stop before the last element so that the last one is easily recognized as never spoken
    // before.
    let mut mem: HashMap<_, _> = input[..input.len() - 1]
        .iter()
        .copied()
        .enumerate()
        .map(|(i, n)| (n, i + 1)) // turns are 1-indexed
        .collect();

    // Use the entry API to avoid multiple lookups and fold to track the last_num
    let result = (input.len()..turns).fold(*input.last().unwrap(), |last_num, turn| {
        // Compute the next number
        match mem.entry(last_num) {
            Entry::Occupied(mut v) => turn - v.insert(turn),
            Entry::Vacant(v) => {
                v.insert(turn);
                0
            }
        }
    });

    Ok(result)
}

fn main() -> Result<()> {
    // Process input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let nums: Vec<usize> = input
        .trim()
        .split(',')
        .filter_map(|l| l.parse().ok())
        .collect();

    let now = Instant::now();
    match part1(&nums, 2020) {
        Ok(v) => println!("Part 1: {}, took {:#?}", v, now.elapsed()),
        Err(e) => eprintln!("{}", e),
    };

    let now = Instant::now();
    match part1(&nums, 30_000_000) {
        Ok(v) => println!("Part 2: {}, took {:#?}", v, now.elapsed()),
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
