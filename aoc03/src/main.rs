use std::io::{self, Read};
use std::str::FromStr;
use std::time::Instant;

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
    fn iter_slope(&self, x_step: usize, y_step: usize) -> IterSlope {
        IterSlope {
            slope: self,
            x: 0,
            y: 0,
            x_step,
            y_step,
        }
    }
}

struct IterSlope<'a> {
    slope: &'a Slope,
    x: usize,
    y: usize,
    x_step: usize,
    y_step: usize,
}

impl<'a> Iterator for IterSlope<'a> {
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

        self.y += self.y_step;
        self.x += self.x_step;

        result
    }
}

fn part1(slope: &Slope) -> Result<()> {
    let count: u32 = slope.iter_slope(3, 1).map(|b| b as u32).sum();
    println!("Part 1: {}", count);
    Ok(())
}

fn part2(slope: &Slope) -> Result<()> {
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let product: u32 = slopes
        .iter()
        .map(|(x_step, y_step)| {
            slope
                .iter_slope(*x_step, *y_step)
                .map(|b| b as u32)
                .sum::<u32>()
        })
        .product();

    println!("Part 2: {}", product);
    Ok(())
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let slope: Slope = input.parse()?;

    let now = Instant::now();
    part1(&slope)?;
    println!("Part 1 took: {:#?}", now.elapsed());
    let now = Instant::now();
    part2(&slope)?;
    println!("Part 2 took: {:#?}", now.elapsed());

    Ok(())
}
