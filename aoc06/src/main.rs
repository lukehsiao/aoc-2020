use std::collections::HashSet;
use std::io::{self, Read};
use std::time::Instant;

use anyhow::Result;

fn count(input: &str) -> Result<usize> {
    let mut total: HashSet<char> = HashSet::new();

    for line in input.lines() {
        for c in line.chars() {
            total.insert(c);
        }
    }

    Ok(total.len())
}

fn part1(input: &str) -> Result<()> {
    // Blank line separates groups
    let customs: Vec<usize> = input.split("\n\n").filter_map(|l| count(l).ok()).collect();

    println!("Part 1: {}", customs.iter().sum::<usize>());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() -> Result<()> {
        assert_eq!(3, count("abc")?);
        assert_eq!(3, count("a\nb\nc\n")?);

        Ok(())
    }
}
