use std::io::{self, Read};
use std::str::FromStr;
use std::time::Instant;

use anyhow::{anyhow, Error, Result};

#[derive(Debug)]
struct Input {
    target: isize,
    buses: Vec<isize>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Input> {
        let target = s.lines().next().unwrap().parse()?;
        let buses: Vec<isize> = s
            .lines()
            .skip(1)
            .next()
            .unwrap()
            .split(',')
            .filter_map(|c| match c {
                "x" => None,
                n => n.parse().ok(),
            })
            .collect();

        Ok(Input { target, buses })
    }
}

fn part1(input: &Input) -> Result<isize> {
    let mut min_wait = isize::MAX;
    let mut bus_id = 0;

    for (idx, id) in input.buses.iter().enumerate() {
        let mut multiplier = 1;
        while id * multiplier < input.target {
            multiplier += 1;
        }

        let wait = (id * multiplier) - input.target;
        if wait < min_wait {
            bus_id = input.buses[idx];
            min_wait = wait;
        }
    }
    Ok(bus_id * min_wait)
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
