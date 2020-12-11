use std::io::{self, Read};
use std::time::Instant;

use anyhow::Result;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Spot {
    Occupied,
    Empty,
    Floor,
}

#[derive(Debug)]
struct Seating {
    spots: Vec<Vec<Spot>>,
    iter: usize,
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1), // top left
    (-1, 0),  // top
    (-1, 1),  // top right
    (0, -1),  // left
    (0, 1),   // right
    (1, -1),  // bottom left
    (1, 0),   // bottom
    (1, 1),   // bottom right
];

impl Seating {
    fn new(input: &str) -> Seating {
        let mut seating: Seating = Seating {
            spots: vec![],
            iter: 0,
        };

        for line in input.lines() {
            let row: Vec<Spot> = line
                .chars()
                .map(|c| match c {
                    'L' => Spot::Empty,
                    '#' => Spot::Occupied,
                    '.' => Spot::Floor,
                    _ => panic!("Invalid input: \"{}\"", line),
                })
                .collect();
            seating.spots.push(row);
        }

        seating
    }

    fn on_grid(&self, row: isize, col: isize) -> bool {
        col >= 0
            && col < self.spots[0].len() as isize
            && row >= 0
            && row < self.spots.len() as isize
    }

    fn occupied_neighbors(&self, row: isize, col: isize) -> u32 {
        DIRECTIONS
            .iter()
            .map(|(drow, dcol)| {
                if self.on_grid(row + drow, col + dcol)
                    && self.spots[(row + drow) as usize][(col + dcol) as usize] == Spot::Occupied
                {
                    1
                } else {
                    0
                }
            })
            .sum()
    }

    // Perform one iteration of the rules
    fn evolve(&mut self) -> bool {
        let mut next = self.spots.clone();

        let mut changed = false;

        for row in 0..self.spots.len() {
            for col in 0..self.spots[0].len() {
                match self.spots[row][col] {
                    Spot::Floor => continue,
                    Spot::Empty => {
                        if self.occupied_neighbors(row as isize, col as isize) == 0 {
                            next[row][col] = Spot::Occupied;
                            changed = true;
                        }
                    }
                    Spot::Occupied => {
                        if self.occupied_neighbors(row as isize, col as isize) >= 4 {
                            next[row][col] = Spot::Empty;
                            changed = true;
                        }
                    }
                }
            }
        }

        if changed {
            self.iter += 1;
        }

        self.spots = next;
        changed
    }
}

fn part1(seating: &mut Seating) -> Result<usize> {
    while seating.evolve() {}

    let occupied = seating
        .spots
        .iter()
        .map(|row| {
            row.iter()
                .map(|s| if *s == Spot::Occupied { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum();

    Ok(occupied)
}

fn main() -> Result<()> {
    // Process input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut seating = Seating::new(&input);

    let now = Instant::now();
    match part1(&mut seating) {
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
