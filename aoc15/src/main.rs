use std::collections::HashMap;
use std::io::{self, Read};
use std::time::Instant;

use anyhow::Result;

fn part1(input: &Vec<usize>, turns: usize) -> Result<usize> {
    let mut mem: HashMap<usize, Vec<usize>> = HashMap::new();

    // First, fill in the input turns
    for (idx, num) in input.iter().enumerate() {
        if let Some(v) = mem.get_mut(num) {
            v.push(idx + 1);
        } else {
            mem.insert(*num, vec![idx + 1]);
        }
    }

    let mut last_num: usize = input[input.len() - 1];
    for i in input.len() + 1..=turns {
        if let Some(j) = mem.get(&last_num) {
            // First time it's been spoken
            if j.len() == 1 {
                last_num = 0;
            } else {
                last_num = j[1] - j[0];
            }

            if let Some(k) = mem.get_mut(&last_num) {
                k.push(i);
                if k.len() > 2 {
                    k.remove(0);
                }
            } else {
                mem.insert(last_num, vec![i]);
            }
        } else {
            last_num = 0;
            mem.insert(last_num, vec![i]);
        }
    }

    Ok(last_num)
}

fn main() -> Result<()> {
    // Process input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let nums: Vec<usize> = input
        .trim()
        .split(',')
        .filter_map(|l| l.parse().ok())
        .collect();

    let now = Instant::now();
    match part1(&nums, 2020) {
        Ok(v) => println!("Part 1: {}, took {:#?}", v, now.elapsed()),
        Err(e) => eprintln!("{}", e),
    };

    let now = Instant::now();
    match part1(&nums, 30_000_000) {
        Ok(v) => println!("Part 2: {}, took {:#?}", v, now.elapsed()),
        Err(e) => eprintln!("{}", e),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() -> Result<()> {
        Ok(())
    }
}
