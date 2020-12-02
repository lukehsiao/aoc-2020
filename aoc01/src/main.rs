use std::collections::HashSet;
use std::io::{self, Read};

use anyhow::Result;

// This is just TwoSum
fn part1(nums: &[i64], target: i64) -> Result<HashSet<i64>> {
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

// This is 3Sum. In this approach, I'm trying to reuse the work of 2 sum to make
// the code simpler. Probably isn't the best way.
fn part2(nums: &mut Vec<i64>, target: i64) -> Result<HashSet<i64>> {
    let mut result: HashSet<i64> = HashSet::new();
    nums.sort();

    for i in 2..nums.len() {
        let sub_target: i64 = target - nums[i];
        let two_sum = part1(&nums[..i], sub_target)?;
        if two_sum.len() == 2 {
            two_sum.iter().for_each(|n| {
                let _ = result.insert(*n);
            });
            result.insert(nums[i]);
            break;
        }
    }

    Ok(result)
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut nums: Vec<i64> = input.lines().map(|n| n.parse::<i64>().unwrap()).collect();

    println!("Part 1: {}", part1(&nums, 2020)?.iter().product::<i64>());
    println!(
        "Part 2: {}",
        part2(&mut nums, 2020)?.iter().product::<i64>()
    );

    Ok(())
}
