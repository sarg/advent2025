use itertools::Itertools;
use sscanf::sscanf;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Point = (i64, i64);

fn area(p1: &Point, p2: &Point) -> i64 {
    (p1.0 - p2.0 + 1).abs() * (p1.1 - p2.1 + 1).abs()
}

fn main() -> std::io::Result<()> {
    let file = File::open("input/9")?;
    let points: Vec<Point> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|l| sscanf!(l, "{i64},{i64}"))
        .filter_map(Result::ok)
        .collect();

    let part1 = (0..points.len())
        .tuple_combinations()
        .map(|(a, b)| area(&points[a], &points[b]))
        .max()
        .unwrap();

    println!("Part 1: {part1}");

    Ok(())
}
