use std::io::{self, Read};
use std::time::Instant;

use anyhow::{anyhow, Result};

fn parse_input(input: &str) -> Result<Vec<(char, isize)>> {
    let result = input
        .lines()
        .map(|l| {
            (
                l.chars().next().unwrap(),
                l.get(1..).unwrap().parse().unwrap(),
            )
        })
        .collect();

    Ok(result)
}

fn part1(actions: &Vec<(char, isize)>) -> Result<isize> {
    const DIRECTIONS: [char; 4] = ['N', 'E', 'S', 'W'];
    let (mut x, mut y) = (0, 0);
    let mut dir_idx = 1;

    for action in actions {
        match action.0 {
            'N' => y += action.1,
            'S' => y -= action.1,
            'E' => x += action.1,
            'W' => x -= action.1,
            'L' => match action.1 {
                90 => dir_idx = (dir_idx + 3) % 4,
                180 => dir_idx = (dir_idx + 2) % 4,
                270 => dir_idx = (dir_idx + 1) % 4,
                0 | 360 => (),
                _ => return Err(anyhow!("Unexpected rotation: \"{:#?}\"", action)),
            },
            'R' => match action.1 {
                90 => dir_idx = (dir_idx + 1) % 4,
                180 => dir_idx = (dir_idx + 2) % 4,
                270 => dir_idx = (dir_idx + 3) % 4,
                0 | 360 => (),
                _ => return Err(anyhow!("Unexpected rotation: \"{:#?}\"", action)),
            },
            'F' => match DIRECTIONS[dir_idx] {
                'N' => y += action.1,
                'S' => y -= action.1,
                'E' => x += action.1,
                'W' => x -= action.1,
                _ => (),
            },
            _ => return Err(anyhow!("Unexpected action: {:#?}", action)),
        }
    }

    Ok(x.abs() + y.abs())
}

fn main() -> Result<()> {
    // Process input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let actions = parse_input(&input)?;

    let now = Instant::now();
    match part1(&actions) {
        Ok(v) => println!("Part 1: {}, took {:#?}", v, now.elapsed()),
        Err(e) => eprintln!("{}", e),
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() -> Result<()> {
        let input = "F10\nN3\nF7\nR90\nF11";
        let actions = parse_input(input)?;

        assert_eq!(('F', 7), actions[2]);

        Ok(())
    }
}
