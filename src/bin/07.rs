use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input/7")?;
    let mut lines_iter = BufReader::new(file).lines();

    let start = lines_iter.next().unwrap().unwrap();
    let s_idx = start.find('S').expect("Start must be there");

    let mut row = start.into_bytes();
    row[s_idx] = b'|';

    let mut reachable: Vec<u64> = vec![0; row.len()];
    reachable[s_idx] = 1;
    let mut splits = 0;
    for line in lines_iter {
        line?.chars().enumerate().for_each(|(i, c)| {
            if c == '^' && row[i] == b'|' {
                splits += 1;

                reachable[i - 1] += reachable[i];
                reachable[i + 1] += reachable[i];
                reachable[i] = 0;

                row[i] = b'.';
                row[i - 1] = b'|';
                row[i + 1] = b'|';
            }
        });
    }
    println!("Part 1: {splits}");
    println!("Part 2: {}", reachable.iter().sum::<u64>());

    Ok(())
}
