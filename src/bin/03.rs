use std::cmp;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn joltage(cells: Vec<u32>) -> u32 {
    cells
        .into_iter()
        .fold((0, 0), |(m, p), v| {
            (cmp::max(m, p * 10 + v), cmp::max(p, v))
        })
        .0
}

fn main() -> std::io::Result<()> {
    let file = File::open("input/3")?;
    let reader = BufReader::new(file);

    let mut part1 = 0;
    for line in reader.lines() {
        let line_content = line?;
        let cells: Vec<u32> = line_content
            .chars()
            .map(|c| c.to_digit(10).expect("digit"))
            .collect();

        let j = joltage(cells);
        //println!("{line_content} => {j}");
        part1 += j;
    }

    println!("Part1: {part1}");
    Ok(())
}
