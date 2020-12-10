use std::io::{self, Read};
use std::time::Instant;

use anyhow::{anyhow, Result};

fn parse_input(input: &str) -> Result<Vec<usize>> {
    let result: Vec<usize> = input.lines().filter_map(|l| l.parse().ok()).collect();

    Ok(result)
}

fn find_pair(target: usize, nums: &[usize]) -> Option<(usize, usize)> {
    nums.iter().enumerate().find_map(|(i, num_1)| {
        let pair = if target > *num_1 {
            target - num_1
        } else {
            num_1 - target
        };
        if nums.iter().skip(i + 1).any(|num_2| *num_2 == pair) {
            Some((*num_1, pair))
        } else {
            None
        }
    })
}

fn part1(nums: &[usize], window_len: usize) -> Result<usize> {
    // Look at each window of len 25 + 1
    for window in nums.windows(window_len + 1) {
        let target = window.last().unwrap();

        match find_pair(*target, &window[0..window.len() - 1]) {
            Some(_) => {}
            None => return Ok(*target),
        }
    }

    Err(anyhow!("No solution found."))
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let nums = parse_input(&input)?;

    let now = Instant::now();
    match part1(&nums, 25) {
        Ok(v) => println!("Part 1: {}, took {:#?}", v, now.elapsed()),
        Err(e) => eprintln!("{}", e),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = parse_input(
            "
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576",
        )?;

        assert_eq!(127, part1(&input, 5)?);

        Ok(())
    }

    #[test]
    fn test_find_pair() -> Result<()> {
        let nums = vec![1, 2, 3, 4, 5];

        assert_eq!(Some((4, 5)), find_pair(9, &nums));

        Ok(())
    }
}
