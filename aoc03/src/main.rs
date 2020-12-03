use std::io::{self, Read};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

/// Struct to represent the input as 2D vectors
#[derive(Debug)]
struct Slope {
    trees: Vec<Vec<bool>>,
}

impl FromStr for Slope {
    type Err = Error;

    fn from_str(s: &str) -> Result<Slope> {
        let mut trees: Vec<Vec<bool>> = vec![];
        for line in s.lines() {
            let mut row: Vec<bool> = vec![];
            for c in line.chars() {
                // If there is a tree, it is true
                match c {
                    '.' => row.push(false),
                    '#' => row.push(true),
                    _ => return Err(anyhow!("Invalid char in input: {}", c)),
                }
            }
            trees.push(row);
        }
        Ok(Slope { trees })
    }
}

impl Slope {
    fn iter_part1(&self) -> IterPart1 {
        IterPart1 {
            slope: self,
            x: 0,
            y: 0,
        }
    }
}

struct IterPart1<'a> {
    slope: &'a Slope,
    x: usize,
    y: usize,
}

impl<'a> Iterator for IterPart1<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        let result = if let Some(next_row) = self.slope.trees.get(self.y) {
            // Wrap next-row if necessary
            let x = match self.x {
                0 => 0,
                n => n % next_row.len(),
            };
            if next_row[x] {
                Some(true)
            } else {
                Some(false)
            }
        } else {
            None
        };

        self.y += 1;
        self.x += 3;

        result
    }
}

fn part1(slope: &Slope) -> Result<()> {
    let count: u32 = slope.iter_part1().map(|b| b as u32).sum();

    println!("Part 1: {}", count);
    Ok(())
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let slope: Slope = input.parse()?;

    part1(&slope)?;

    Ok(())
}
