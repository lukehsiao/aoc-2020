use std::collections::HashSet;
use std::io::{self, Read};
use std::time::Instant;

use anyhow::{anyhow, Result};

fn count_any(input: &str) -> Result<usize> {
    let mut total: HashSet<char> = HashSet::new();

    for line in input.lines() {
        for c in line.chars() {
            total.insert(c);
        }
    }

    Ok(total.len())
}

fn count_all(input: &str) -> Result<usize> {
    let init_set: HashSet<char> = match input.lines().next() {
        Some(l) => l.chars().collect(),
        None => return Err(anyhow!("Invalid input: \"{}\"", input)),
    };

    let count_all: HashSet<char> = input
        .lines()
        .map(|l| l.chars().collect::<HashSet<char>>())
        .fold(init_set, |acc, x| acc.intersection(&x).cloned().collect());

    Ok(count_all.len())
}

fn part1(input: &str) -> Result<()> {
    // Blank line separates groups
    let customs: Vec<usize> = input
        .split("\n\n")
        .filter_map(|l| count_any(l).ok())
        .collect();

    println!("Part 1: {}", customs.iter().sum::<usize>());
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    // Blank line separates groups
    let customs: Vec<usize> = input
        .split("\n\n")
        .filter_map(|l| count_all(l).ok())
        .collect();

    println!("Part 2: {}", customs.iter().sum::<usize>());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_any() -> Result<()> {
        assert_eq!(3, count_any("abc")?);
        assert_eq!(3, count_any("a\nb\nc\n")?);

        Ok(())
    }

    #[test]
    fn test_count_all() -> Result<()> {
        assert_eq!(1, count_all("a\nabc\na\n")?);
        assert_eq!(3, count_all("abc")?);
        assert_eq!(0, count_all("a\nb\nc\n")?);
        assert_eq!(1, count_all("a\na\na\n")?);

        Ok(())
    }
}
