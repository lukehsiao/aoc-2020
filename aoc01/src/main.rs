use std::collections::HashMap;
use std::io::{self, Read};

use anyhow::Result;

const TARGET: i64 = 2020;

// This is just TwoSum
fn part1(input: &str) -> Result<()> {
    let mut map = HashMap::new();

    let mut result = 0;

    for line in input.lines() {
        let num: i64 = line.parse()?;
        if map.contains_key(&num) {
            result = num * map.get(&num).unwrap();
        } else {
            map.insert(TARGET - num, num);
        }
    }

    println!("Part 1: {}", result);
    Ok(())
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    Ok(())
}
