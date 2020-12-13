use std::convert::TryInto;
use std::io::{self, Read};
use std::str::FromStr;
use std::time::Instant;

use anyhow::{Error, Result};
use num::Integer;

#[derive(Debug)]
struct Input {
    target: isize,
    buses: Vec<(usize, isize)>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Input> {
        let target = s.lines().next().unwrap().parse()?;
        let buses: Vec<(usize, isize)> = s
            .lines()
            .skip(1)
            .next()
            .unwrap()
            .split(',')
            .enumerate()
            .filter_map(|(i, c)| match c {
                "x" => None,
                n => Some((i, n.parse().unwrap())),
            })
            .collect();

        Ok(Input { target, buses })
    }
}

fn part1(input: &Input) -> Result<isize> {
    let mut min_wait = isize::MAX;
    let mut bus_id = 0;

    for (_, id) in input.buses.iter() {
        if *id == 0 {
            continue;
        }
        let mut multiplier = 1;
        while id * multiplier < input.target {
            multiplier += 1;
        }

        let wait = (id * multiplier) - input.target;
        if wait < min_wait {
            bus_id = *id;
            min_wait = wait;
        }
    }
    Ok(bus_id * min_wait)
}

fn part2(input: &Input) -> Result<usize> {
    let mut t = 1;
    let mut lcm = 1;

    // Naive implementation just increment and test brute force
    for (idx, bus_id) in input.buses.iter() {
        // Increment time until we find a time that works for this bus.
        loop {
            if (t + idx) % *bus_id as usize == 0 {
                // This is a valid time that matches the expected offset
                break;
            }
            t += lcm
        }

        // This is the trick. We shouldn't just increment by 1, we should increment in steps that
        // keep a valid offset for this (and all previous) buses. This is least common multiple of
        // all of the buses' IDs.
        lcm = lcm.lcm(&(*bus_id as usize));
    }

    Ok(t)
}

fn main() -> Result<()> {
    // Process input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input = input.parse()?;

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
