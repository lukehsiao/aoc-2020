use std::collections::HashSet;
use std::io::{self, Read};
use std::time::Instant;

use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq)]
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

fn part1(input: &str) -> Result<()> {
    let mut accumulator = 0;
    let instructions = parse_input(input)?;
    let mut visited: HashSet<i32> = HashSet::new();
    let mut pc: i32 = 0;

    while pc >= 0 && pc < instructions.len() as i32 {
        let instruction = &instructions[pc as usize];
        if visited.contains(&pc) {
            println!("Part 1: {}", accumulator);
            break;
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

    Ok(())
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let now = Instant::now();
    part1(&input)?;
    println!("Part 1 took: {:#?}", now.elapsed());

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
