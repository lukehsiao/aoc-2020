use std::io::{self, Read};
use std::str::FromStr;
use std::time::Instant;

use anyhow::{anyhow, Error, Result};

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

fn part2(input: &Input) -> Result<isize> {
    todo!()
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
