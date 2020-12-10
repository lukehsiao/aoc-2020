use std::collections::HashSet;
use std::io::{self, Read};
use std::time::Instant;

use anyhow::{anyhow, Result};

fn parse_input(input: &str) -> Result<Vec<usize>> {
    let result: Vec<usize> = input.lines().filter_map(|l| l.parse().ok()).collect();

    Ok(result)
}

fn part1(nums: &Vec<usize>) -> Result<usize> {
    todo!()
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let nums = parse_input(&input)?;

    let now = Instant::now();
    part1(&nums)?;
    println!("Part 1 took: {:#?}", now.elapsed());

    Ok(())
}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
// }
