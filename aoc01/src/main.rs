use std::collections::HashSet;
use std::io::{self, Read};

use anyhow::Result;

// This is just TwoSum
fn part1(nums: &Vec<i64>, target: i64) -> Result<HashSet<i64>> {
    let mut dual: HashSet<i64> = HashSet::new();
    let mut result: HashSet<i64> = HashSet::new();

    for num in nums {
        if dual.contains(num) {
            result.insert(*num);
            result.insert(target - num);
            break;
        } else {
            dual.insert(target - num);
        }
    }

    Ok(result)
}

// This is just ThreeSum
fn part2(nums: &mut Vec<i64>) -> Result<()> {
    let result = 0;
    nums.sort();

    println!("Part 2: {}", result);
    Ok(())
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut nums: Vec<i64> = input.lines().map(|n| n.parse::<i64>().unwrap()).collect();

    println!("Part 1: {}", part1(&nums, 2020)?.iter().product::<i64>());
    part2(&mut nums)?;
    Ok(())
}
