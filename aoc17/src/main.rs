use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{self, Read};
use std::time::Instant;

use anyhow::{anyhow, Result};
use itertools::Itertools;

#[derive(Debug)]
struct Conway {
    cubes: HashMap<Vec<isize>, bool>,
    iter: isize,
    init_dim: isize,
    dim: isize,
    directions: Vec<Vec<isize>>,
}

impl Conway {
    fn new(input: &str, dim: isize) -> Result<Conway> {
        // Assume a square input
        let mut cubes: HashMap<_, _> = HashMap::new();
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
                let mut coords = vec![x.try_into().unwrap(), y.try_into().unwrap()];
                for _ in 2..dim {
                    coords.push(0);
                }

                match c {
                    '#' => cubes.insert(coords, true),
                    '.' => cubes.insert(coords, false),
                    _ => return Err(anyhow!("Invalid input: \"{}\"", line)),
                };
            }
        }

        Ok(Conway {
            cubes,
            iter: 1,
            init_dim,
            dim,
            directions: (0..dim)
                .map(|_| (-1..=1))
                .multi_cartesian_product()
                .into_iter()
                .filter(|v| !v.iter().all(|n| *n == 0))
                .collect(),
        })
    }

    fn occupied_neighbors(&self, coords: &[isize]) -> usize {
        self.directions
            .iter()
            .map(|v| {
                let key: Vec<isize> = v.iter().zip(coords).map(|(v, c)| c + *v).collect();
                match self.cubes.get(&key) {
                    Some(true) => 1,
                    _ => 0,
                }
            })
            .sum()
    }

    // Perform one iteration of the part 1 rules
    fn evolve(&mut self) {
        let mut next = self.cubes.clone();

        for x in -self.iter..=self.init_dim + self.iter {
            for y in -self.iter..=self.init_dim + self.iter {
                for z in -self.iter..=self.iter {
                    let coords = vec![x, y, z];
                    match next.entry(coords.clone()) {
                        Entry::Occupied(mut v) => match v.get() {
                            true => match self.occupied_neighbors(&coords) {
                                2 | 3 => (),
                                _ => {
                                    let _ = v.insert(false);
                                }
                            },
                            false => match self.occupied_neighbors(&coords) {
                                3 => {
                                    let _ = v.insert(true);
                                }
                                _ => (),
                            },
                        },
                        Entry::Vacant(v) => match self.occupied_neighbors(&coords) {
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

    // Perform one iteration of the part 1 rules
    fn evolve2(&mut self) {
        let mut next = self.cubes.clone();

        for x in -self.iter..=self.init_dim + self.iter {
            for y in -self.iter..=self.init_dim + self.iter {
                for z in -self.iter..=self.iter {
                    for w in -self.iter..=self.iter {
                        let coords = vec![x, y, z, w];
                        match next.entry(coords.clone()) {
                            Entry::Occupied(mut v) => match v.get() {
                                true => match self.occupied_neighbors(&coords) {
                                    2 | 3 => (),
                                    _ => {
                                        let _ = v.insert(false);
                                    }
                                },
                                false => match self.occupied_neighbors(&coords) {
                                    3 => {
                                        let _ = v.insert(true);
                                    }
                                    _ => (),
                                },
                            },
                            Entry::Vacant(v) => match self.occupied_neighbors(&coords) {
                                3 => {
                                    let _ = v.insert(true);
                                }
                                _ => (),
                            },
                        }
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

fn part2(input: &mut Conway) -> Result<usize> {
    for _ in 0..6 {
        input.evolve2();
    }

    Ok(input.count_occupied())
}

fn main() -> Result<()> {
    // Process input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut conway = Conway::new(&input, 3)?;
    let now = Instant::now();
    match part1(&mut conway) {
        Ok(v) => println!("Part 1: {}, took {:#?}", v, now.elapsed()),
        Err(e) => eprintln!("{}", e),
    };

    let mut conway = Conway::new(&input, 4)?;
    let now = Instant::now();
    match part2(&mut conway) {
        Ok(v) => println!("Part 2: {}, took {:#?}", v, now.elapsed()),
        Err(e) => eprintln!("{}", e),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_itertools() -> Result<()> {
        let combos = (0..=3).map(|_| (-1..=1)).multi_cartesian_product();
        combos.clone().for_each(|c| {
            let _ = dbg!(&c);
        });
        assert_eq!(27, combos.count());
        Ok(())
    }
}
