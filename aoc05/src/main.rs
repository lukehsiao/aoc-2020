use std::io::{self, Read};
use std::str::FromStr;
use std::time::Instant;

use anyhow::{anyhow, Error, Result};

#[derive(Debug)]
struct Seat {
    row: u8,
    col: u8,
    id: u32,
}

impl FromStr for Seat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Seat> {
        let row = 0;
        let col = 0;
        let id = 0;

        Ok(Seat { row, col, id })
    }
}

fn part1(input: &str) -> Result<()> {
    // Blank line separates entries
    let seats: Vec<Seat> = input.lines().filter_map(|l| l.parse().ok()).collect();

    println!("Part 1: {}", passports.len());
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
