use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn part1_args(strs: &Vec<&str>) -> Vec<u64> {
    strs.iter()
        .map(|v| v.trim().parse::<u64>().unwrap())
        .collect()
}

fn part2_args(strs: &Vec<&str>) -> Vec<u64> {
    strs.iter()
        .flat_map(|a| a.chars().rev().enumerate())
        .map(|(i, v)| (i, v.to_digit(10).map(|e| e as u64)))
        .fold(Vec::<u64>::new(), |mut args, (i, d)| {
            match (args.get(i), d) {
                (Some(n), Some(d)) => args[i] = n * 10 + d,
                (None, Some(d)) => args.push(d),
                (None, None) => args.push(0),
                _ => {}
            }
            args
        })
}

fn main() -> std::io::Result<()> {
    let file = File::open("input/6")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines().map(Result::unwrap).collect_vec();
    let last_line = lines.pop().unwrap();
    let mut ops = last_line.as_str();

    let mut part1 = 0;
    let mut part2 = 0;
    let mut from = 0;
    while !ops.is_empty() {
        let (len, next) = ops[1..]
            .find(|c| c != ' ')
            .map_or((ops.len(), ops.len()), |v| (v, v + 1));
        let op_char = ops.chars().next().expect("should be present");
        let op = |a: u64, b: u64| -> u64 {
            match op_char {
                '+' => a + b,
                '*' => a * b,
                _ => unreachable!(),
            }
        };

        let args = lines.iter().map(|l| &l[from..(from + len)]).collect_vec();
        part1 += part1_args(&args).into_iter().reduce(op).unwrap();
        part2 += part2_args(&args).into_iter().reduce(op).unwrap();

        from += next;
        ops = &ops[next..];
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}
