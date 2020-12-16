use std::io::{self, Read};
use std::ops::RangeInclusive;
use std::time::Instant;

use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Rule {
    lower: RangeInclusive<usize>,
    upper: RangeInclusive<usize>,
}

impl Rule {
    fn contains(&self, n: usize) -> bool {
        self.lower.contains(&n) || self.upper.contains(&n)
    }
}

fn parse_rules(input: &str) -> Result<Vec<Rule>> {
    Ok(input
        .lines()
        .map(|l| {
            lazy_static! {
                static ref RULE: Regex =
                    Regex::new(r"^(?P<name>[\w\s]+):\s+(?P<r1>[\d-]+) or (?P<r2>[\d-]+)").unwrap();
            }

            let caps = match RULE.captures(l) {
                None => panic!("No match: {}", l),
                Some(c) => c,
            };

            let r1_parts: Vec<usize> = caps["r1"].split('-').map(|s| s.parse().unwrap()).collect();
            let lower = r1_parts[0]..=r1_parts[1];

            let r2_parts: Vec<usize> = caps["r2"].split('-').map(|s| s.parse().unwrap()).collect();
            let upper = r2_parts[0]..=r2_parts[1];

            Rule { lower, upper }
        })
        .collect())
}

fn part1(input: &str) -> Result<usize> {
    let sections: Vec<_> = input.split("\n\n").collect();

    let rules = parse_rules(sections[0])?;

    // Check all nearby tickets only
    let ticket_error_rate = sections[2].lines().skip(1).fold(0, |mut acc, ticket| {
        for value in ticket
            .trim()
            .split(',')
            .map(|v| v.parse::<usize>().unwrap())
        {
            if rules.iter().all(|rule| !rule.contains(value)) {
                acc += value
            }
        }
        acc
    });

    Ok(ticket_error_rate)
}

fn part2(input: &str) -> Result<usize> {
    let sections: Vec<_> = input.split("\n\n").collect();

    let rules = parse_rules(sections[0])?;

    dbg!(&rules);

    todo!()
}

fn main() -> Result<()> {
    // Process input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let now = Instant::now();
    match part1(&input) {
        Ok(v) => println!("Part 1: {}, took {:#?}", v, now.elapsed()),
        Err(e) => eprintln!("{}", e),
    };

    let now = Instant::now();
    match part2(&input) {
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
