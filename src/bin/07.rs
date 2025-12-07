use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input/7")?;
    let mut lines_iter = BufReader::new(file).lines();

    let start = lines_iter.next().unwrap().unwrap();
    let mut row = vec![0 as u64; start.len()];
    row[start.find('S').expect("Start must be there")] = 1;

    let mut splits = 0;
    for line in lines_iter {
        line?.chars().enumerate().for_each(|(i, c)| {
            if c == '^' && row[i] > 0 {
                splits += 1;
                row[i - 1] += row[i];
                row[i + 1] += row[i];
                row[i] = 0;
            }
        });
    }
    println!("Part 1: {splits}");
    println!("Part 2: {}", row.iter().sum::<u64>());

    Ok(())
}
