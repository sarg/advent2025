use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use itertools::Itertools;

struct Machine {
    target: usize,
    buttons: Vec<usize>,
    joltage: Vec<usize>,
}

fn parse_indicators(s: &str) -> usize {
    s[1..s.len() - 1]
        .bytes()
        .enumerate()
        .filter_map(|(i, b)| if b == b'#' { Some(1 << i) } else { None })
        .sum()
}

fn parse_buttons(s: &str) -> usize {
    s[1..s.len() - 1]
        .split(',')
        .map(|n| n.parse::<usize>())
        .filter_map(Result::ok)
        .map(|b| 1 << b)
        .sum()
}

impl Machine {
    fn new(s: String) -> Self {
        let mut siter = s.split_ascii_whitespace();

        Self {
            target: siter.next().map(parse_indicators).unwrap(),

            buttons: siter
                .take_while_ref(|s| s.starts_with('('))
                .map(parse_buttons)
                .collect(),

            joltage: siter
                .next()
                .map(|s| {
                    s[1..s.len() - 1]
                        .split(',')
                        .map(str::parse::<usize>)
                        .filter_map(Result::ok)
                        .collect_vec()
                })
                .unwrap(),
        }
    }

    fn configure(&self) -> usize {
        let mut reachable = BTreeSet::from_iter(self.buttons.clone().into_iter());
        let mut n = 1;

        while !reachable.contains(&self.target) {
            n += 1;

            reachable
                .clone()
                .iter()
                .flat_map(|r| self.buttons.iter().map(move |b| r ^ b))
                .for_each(|r| {
                    reachable.insert(r);
                });
        }
        n
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input/10")?;
    let machines: Vec<Machine> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(Machine::new)
        .collect();

    let part1 = machines.iter().map(|m| m.configure()).sum::<usize>();

    println!("Part 1: {part1}");

    Ok(())
}
