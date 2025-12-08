use itertools::Itertools;
use sscanf::sscanf;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Point = (i64, i64, i64);

fn dist(p1: &Point, p2: &Point) -> u64 {
    let d2 = (p1.0 - p2.0) * (p1.0 - p2.0)
        + (p1.1 - p2.1) * (p1.1 - p2.1)
        + (p1.2 - p2.2) * (p1.2 - p2.2);
    (d2 as u64).isqrt()
}

fn main() -> std::io::Result<()> {
    let file = File::open("input/8")?;
    let points: Vec<Point> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|l| sscanf!(l, "{i64},{i64},{i64}"))
        .filter_map(Result::ok)
        .collect();

    let mut connects: Vec<usize> = (0..points.len()).collect();
    let mut sizes = HashMap::<usize, usize>::with_capacity(points.len());
    for i in 0..points.len() {
        sizes.insert(i, 1);
    }

    (0..points.len())
        .tuple_combinations()
        .sorted_unstable_by_key(|&(p1, p2)| dist(&points[p1], &points[p2]))
        .take(1000)
        .for_each(|(p1, p2)| {
            let r1 = connects[p1];
            let r2 = connects[p2];
            if r1 != r2 {
                let s2 = sizes.remove(&r2).unwrap();
                let s1 = sizes.remove(&r1).unwrap();
                sizes.insert(r1, s1 + s2);
                for i in 0..connects.len() {
                    if connects[i] == r2 {
                        connects[i] = r1;
                    }
                }
            }
        });

    let part1 = sizes.values().sorted().rev().take(3).product::<usize>();
    println!("Part 1: {part1}");

    Ok(())
}
