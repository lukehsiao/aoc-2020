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

    Ok(one_acc * three_acc)
}

// Approach is to count how many paths there are to reach the end, and accumulate that all the way
// to the front.
fn part2(nums: &[usize], max: usize) -> Result<usize> {
    // Create a bit array where a 1 and index i means the value i was in the input.
    let mut mask: Vec<usize> = vec![0; max + 1];
    mask[0] = 1; // the starting point
    for num in nums {
        mask[*num] = 1;
    }
    mask.append(&mut vec![0, 0, 1]); // for device (which is +3 the max)

    // Start from the back and count the adapters this point can connect to
    for i in (0..max).rev() {
        if mask[i] > 0 {
            mask[i] = mask[i + 1] + mask[i + 2] + mask[i + 3]
        }
    }
    Ok(mask[0])
}

fn main() -> Result<()> {
    // Process input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut nums: Vec<usize> = input.lines().filter_map(|l| l.parse().ok()).collect();
    nums.sort();
    let max = nums.iter().max().unwrap();

    let now = Instant::now();
    match part1(&nums) {
        Ok(v) => println!("Part 1: {}, took {:#?}", v, now.elapsed()),
        Err(e) => eprintln!("{}", e),
    };

    let now = Instant::now();
    match part2(&nums, *max) {
        Ok(v) => println!("Part 2: {}, took {:#?}", v, now.elapsed()),
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
