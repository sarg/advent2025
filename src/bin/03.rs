use std::cmp;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn joltage(mut cells: Vec<u32>) -> u32 {
    let mut max = 0;
    for i in 1..cells.len() {
        let joltage = cells[i - 1] * 10 + cells[i];
        max = cmp::max(max, joltage);
        cells[i] = cmp::max(cells[i], cells[i - 1]);
    }
    return max;
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
