use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input/7")?;
    let mut lines_iter = BufReader::new(file).lines();

    let start = lines_iter.next().unwrap().unwrap().replace("S", "|");
    println!("{start}");

    let mut row = start.into_bytes();
    let mut splits = 0;
    for line in lines_iter {
        line?.chars().enumerate().for_each(|(i, c)| {
            if c == '^' && row[i] == b'|' {
                splits += 1;
                row[i] = b'.';
                row[i - 1] = b'|';
                row[i + 1] = b'|';
            }
        });

        println!("{}", String::from_utf8_lossy(&row));
    }
    println!("Part 1: {splits}");

    Ok(())
}
