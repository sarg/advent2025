use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn part1(data: &Vec<Vec<u64>>, ops: &Vec<char>) -> u64 {
    ops.iter()
        .enumerate()
        .map(|(i, op)| match op {
            '*' => data.iter().map(|d| d[i]).product::<u64>(),
            '+' => data.iter().map(|d| d[i]).sum::<u64>(),
            _ => panic!("Unsupported"),
        })
        .sum()
}

fn main() -> std::io::Result<()> {
    let file = File::open("input/6")?;
    let reader = BufReader::new(file);

    let mut data: Vec<Vec<u64>> = Vec::new();
    let mut ops: Vec<char> = Vec::new();
    for l in reader.lines() {
        let line = l?;
        let t1 = line.split_whitespace().next().unwrap();
        if t1 == "*" {
            ops = line
                .split_whitespace()
                .map(|s| s.chars().next().expect(""))
                .collect_vec();
        } else {
            let tokens = line
                .split_whitespace()
                .map(str::parse::<u64>)
                .map(Result::unwrap)
                .collect_vec();
            data.push(tokens);
        }
    }

    println!("Part 1: {}", part1(&data, &ops));
    println!("Part 2: {}", 0);

    Ok(())
}
