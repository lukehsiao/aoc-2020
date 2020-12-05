use std::io::{self, Read};
use std::str::FromStr;
use std::time::Instant;

use anyhow::{anyhow, Error, Result};

#[derive(Debug)]
struct Seat {
    row: u32,
    col: u32,
    id: u32,
}

impl FromStr for Seat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Seat> {
        let (row, col, id) = decode(s)?;

        Ok(Seat { row, col, id })
    }
}

fn decode(input: &str) -> Result<(u32, u32, u32)> {
    const MULTIPLIER: u32 = 8;
    let mut front = 0;
    let mut back = 127;
    let mut left = 0;
    let mut right = 7;

    for c in input.chars() {
        match c {
            'F' => back = (front + back) / 2,
            'B' => front = (front + back + 1) / 2,
            'L' => right = (left + right) / 2,
            'R' => left = (left + right + 1) / 2,
            _ => return Err(anyhow!("Invalid input: {}", input)),
        }
    }

    let row = match input.chars().nth(6) {
        Some('F') => front,
        Some('B') => back,
        _ => return Err(anyhow!("Invalid input: {}", input)),
    };

    let col = match input.chars().nth(9) {
        Some('R') => right,
        Some('L') => left,
        _ => return Err(anyhow!("Invalid input: {}", input)),
    };

    Ok((row, col, row * MULTIPLIER + col))
}

fn part1(input: &str) -> Result<()> {
    let seats: Vec<Seat> = input.lines().filter_map(|l| l.parse().ok()).collect();

    let max = seats.iter().max_by(|x, y| x.id.cmp(&y.id)).unwrap();

    println!("Part 1: {}", max.id);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut seats: Vec<Seat> = input.lines().filter_map(|l| l.parse().ok()).collect();

    seats.sort_by(|a, b| a.id.cmp(&b.id));

    let mut next = seats[0].id + 1;
    for seat in &seats[1..] {
        if seat.id != next {
            // Found my seat!
            println!("Part 2: {}", next);
            break;
        }
        next += 1;
    }

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
    fn test_decode() -> Result<()> {
        assert_eq!((70, 7, 567), decode("BFFFBBFRRR")?);
        assert_eq!((14, 7, 119), decode("FFFBBBFRRR")?);
        assert_eq!((102, 4, 820), decode("BBFFBBFRLL")?);

        Ok(())
    }
}
