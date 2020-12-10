use std::io::{self, Read};
use std::time::Instant;

use anyhow::{anyhow, Result};

fn part1(nums: &[usize]) -> Result<usize> {
    let mut one_acc = 0;
    let mut three_acc = 0;
    let mut prev = 0;

    for jolt in nums {
        match jolt - prev {
            1 => one_acc += 1,
            2 => {}
            3 => three_acc += 1,
            _ => return Err(anyhow!("Can't use all adapters! {}", jolt)),
        }
        prev = *jolt;
    }
    // Add one for device adapter
    three_acc += 1;

    dbg!(one_acc, three_acc);

    Ok(one_acc * three_acc)
}

fn main() -> Result<()> {
    // Process input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut nums: Vec<usize> = input.lines().filter_map(|l| l.parse().ok()).collect();
    nums.sort();
    // let device_adapter = nums.iter().max().unwrap() + 3;

    let now = Instant::now();
    match part1(&nums) {
        Ok(v) => println!("Part 1: {}, took {:#?}", v, now.elapsed()),
        Err(e) => eprintln!("{}", e),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        Ok(())
    }
}
