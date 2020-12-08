use std::collections::HashSet;
use std::io::{self, Read};
use std::time::Instant;

use anyhow::{anyhow, Result};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn parse_input(input: &str) -> Result<Vec<Instruction>> {
    let mut instructions: Vec<Instruction> = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "nop" => instructions.push(Instruction::Nop(parts[1].parse()?)),
            "acc" => instructions.push(Instruction::Acc(parts[1].parse()?)),
            "jmp" => instructions.push(Instruction::Jmp(parts[1].parse()?)),
            _ => return Err(anyhow!("Invalid instruction: \"{}\"", line)),
        }
    }

    Ok(instructions)
}

fn part1(instructions: &Vec<Instruction>) -> Result<i32> {
    let mut accumulator = 0;
    let mut visited: HashSet<i32> = HashSet::new();
    let mut pc: i32 = 0;

    while pc >= 0 && pc < instructions.len() as i32 {
        let instruction = &instructions[pc as usize];
        if visited.contains(&pc) {
            return Err(anyhow!("Part 1: {}", accumulator));
        }

        visited.insert(pc);
        match instruction {
            Instruction::Nop(_) => pc += 1,
            Instruction::Acc(n) => {
                accumulator += n;
                pc += 1;
            }
            Instruction::Jmp(n) => {
                pc += n;
            }
        }
    }

    Ok(accumulator)
}

// Naive brute force attempt. Literally swap every nop to jmp and jmp to nop until we find the
// solution.
fn part2(instructions: &mut Vec<Instruction>) -> Result<()> {
    let len = instructions.len();
    for i in 0..len {
        let instruction = instructions[i];
        let tmp = match instruction {
            Instruction::Jmp(n) => Instruction::Nop(n),
            Instruction::Nop(n) => Instruction::Jmp(n),
            Instruction::Acc(_) => continue,
        };

        // Swap in the temporary change
        instructions[i] = tmp;

        match part1(&instructions) {
            Ok(n) => {
                println!("Part 2: {}", n);
                break;
            }
            Err(_) => instructions[i] = instruction,
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut instructions = parse_input(&input)?;

    let now = Instant::now();
    if let Err(e) = part1(&instructions) {
        println!("{}", e);
    }
    println!("Part 1 took: {:#?}", now.elapsed());

    let now = Instant::now();
    part2(&mut instructions)?;
    println!("Part 2 took: {:#?}", now.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() -> Result<()> {
        let input = parse_input(
            "nop +0\n\
             acc +1\n\
             jmp +4\n\
             acc +3\n\
             jmp -3\n\
             acc -99\n\
             acc +1\n\
             jmp -4\n\
             acc +6",
        )?;

        assert_eq!(Instruction::Acc(3), input[3]);

        Ok(())
    }
}
