use sscanf::sscanf;
use std::collections::BTreeSet;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::ErrorKind;
use std::process::{Command, Stdio};

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

            let next = reachable
                .iter()
                .flat_map(|r| self.buttons.iter().map(move |b| r ^ b))
                .collect_vec();

            for r in next {
                reachable.insert(r);
            }
        }
        n
    }

    fn set_joltage(&self) -> io::Result<usize> {
        let mut child = Command::new("lp_solve")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .args(["-S1"])
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_fmt(format_args!(
                "min: {};\n\n",
                (0..self.buttons.len()).map(|i| format!("b{i}")).join(" + ")
            ))?;

            for ji in 0..self.joltage.len() {
                let button_equation = self
                    .buttons
                    .iter()
                    .enumerate()
                    .filter(|&(_, b)| (1 << ji) & b > 0)
                    .map(|(bi, _)| format!("b{bi}"))
                    .join(" + ");

                stdin.write_fmt(format_args!(
                    "{} = {};\n",
                    button_equation, self.joltage[ji]
                ))?;
            }

            stdin.write_all(b"\n")?;

            for i in 0..self.buttons.len() {
                stdin.write_fmt(format_args!("int b{i};\n"))?;
            }
        }

        BufReader::new(child.stdout.take().expect("Failed to open stdin"))
            .lines()
            .filter_map(Result::ok)
            .last()
            .and_then(|l| sscanf!(l, "Value of objective function: {f64}").ok())
            .map(|f| f as usize)
            .ok_or(io::Error::new(ErrorKind::InvalidData, "wrong output"))
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

    let part2 = machines
        .iter()
        .map(|m| m.set_joltage())
        .filter_map(Result::ok)
        .sum::<usize>();
    println!("Part 2: {part2}");

    Ok(())
}
