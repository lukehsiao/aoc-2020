use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{self, Read};
use std::time::Instant;

use anyhow::{anyhow, Result};

#[derive(Debug)]
struct Conway {
    cubes: HashMap<(isize, isize, isize), bool>,
    iter: isize,
    init_dim: isize,
}

const DIRECTIONS: [(isize, isize, isize); 26] = [
    (-1, -1, -1),
    (-1, -1, 0),
    (-1, -1, 1),
    (-1, 0, -1),
    (-1, 0, 0),
    (-1, 0, 1),
    (-1, 1, -1),
    (-1, 1, 0),
    (-1, 1, 1),
    (0, -1, -1),
    (0, -1, 0),
    (0, -1, 1),
    (0, 0, -1),
    (0, 0, 1),
    (0, 1, -1),
    (0, 1, 0),
    (0, 1, 1),
    (1, -1, -1),
    (1, -1, 0),
    (1, -1, 1),
    (1, 0, -1),
    (1, 0, 0),
    (1, 0, 1),
    (1, 1, -1),
    (1, 1, 0),
    (1, 1, 1),
];

impl Conway {
    fn new(input: &str) -> Result<Conway> {
        // Assume a square input
        let mut cubes: HashMap<(isize, isize, isize), bool> = HashMap::new();
        let init_dim = input
            .lines()
            .next()
            .unwrap()
            .chars()
            .count()
            .try_into()
            .unwrap();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => cubes.insert((x.try_into().unwrap(), y.try_into().unwrap(), 0), true),
                    '.' => cubes.insert((x.try_into().unwrap(), y.try_into().unwrap(), 0), false),
                    _ => return Err(anyhow!("Invalid input: \"{}\"", line)),
                };
            }
        }

        Ok(Conway {
            cubes,
            iter: 1,
            init_dim,
        })
    }

    fn occupied_neighbors(&self, x: isize, y: isize, z: isize) -> usize {
        DIRECTIONS
            .iter()
            .map(
                |(dx, dy, dz)| match self.cubes.get(&(x + dx, y + dy, z + dz)) {
                    Some(true) => 1,
                    _ => 0,
                },
            )
            .sum()
    }

    // Perform one iteration of the part 1 rules
    fn evolve(&mut self) {
        let mut next = self.cubes.clone();

        for x in -self.iter..=self.init_dim + self.iter {
            for y in -self.iter..=self.init_dim + self.iter {
                for z in -self.iter..=self.iter {
                    match next.entry((x, y, z)) {
                        Entry::Occupied(mut v) => match v.get() {
                            true => match self.occupied_neighbors(x, y, z) {
                                2 | 3 => (),
                                _ => {
                                    let _ = v.insert(false);
                                }
                            },
                            false => match self.occupied_neighbors(x, y, z) {
                                3 => {
                                    let _ = v.insert(true);
                                }
                                _ => (),
                            },
                        },
                        Entry::Vacant(v) => match self.occupied_neighbors(x, y, z) {
                            3 => {
                                let _ = v.insert(true);
                            }
                            _ => (),
                        },
                    }
                }
            }
        }

        self.iter += 1;
        self.cubes = next;
    }

    fn count_occupied(&self) -> usize {
        self.cubes.values().map(|b| *b as usize).sum()
    }
}

fn part1(input: &mut Conway) -> Result<usize> {
    for _ in 0..6 {
        input.evolve();
    }

    Ok(input.count_occupied())
}

fn main() -> Result<()> {
    // Process input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut conway = Conway::new(&input)?;

    let now = Instant::now();
    match part1(&mut conway) {
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
