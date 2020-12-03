use std::collections::HashMap;
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
        let mut trees: Vec<Vec<bool>> = vec![vec![]];
        for line in s.lines() {
            let mut row: Vec<bool> = vec![];
            for c in line.chars() {
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

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let slope: Slope = input.parse()?;
    dbg!(slope);

    Ok(())
}
