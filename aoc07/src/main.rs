use std::collections::HashMap;
use std::io::{self, Read};
use std::time::Instant;

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct InnerBag {
    name: String,
    count: usize,
}

fn parse_rules(input: &str) -> Result<HashMap<String, Vec<InnerBag>>> {
    lazy_static! {
        static ref BAG_RE: Regex = Regex::new(r"^(?P<name>.+?) bags").unwrap();
    }

    lazy_static! {
        static ref INNER_RE: Regex = Regex::new(r"(?P<count>\d+?) (?P<name>.*?) bag").unwrap();
    }
    let mut result: HashMap<String, Vec<InnerBag>> = HashMap::new();

    for line in input.lines() {
        let bag_caps = match BAG_RE.captures(line) {
            None => return Err(anyhow!("No match: {}", line)),
            Some(c) => c,
        };

        let inner: Vec<InnerBag> = INNER_RE
            .captures_iter(line)
            .map(|cap| InnerBag {
                name: cap["name"].parse().unwrap(),
                count: cap["count"].parse().unwrap(),
            })
            .collect();

        result.insert(bag_caps["name"].parse()?, inner);
    }

    Ok(result)
}

fn holds_shiny_gold(rules: &HashMap<String, Vec<InnerBag>>, name: &str) -> bool {
    match rules.get(name) {
        Some(inner) => inner
            .iter()
            .any(|bag| bag.name.as_str() == "shiny gold" || holds_shiny_gold(rules, &bag.name)),
        None => false,
    }
}

fn part1(input: &str) -> Result<()> {
    let rules: HashMap<String, Vec<InnerBag>> = parse_rules(input)?;

    println!(
        "Part 1: {}",
        rules
            .keys()
            .filter(|&rule| holds_shiny_gold(&rules, rule))
            .count()
    );
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
