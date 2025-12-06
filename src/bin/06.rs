use sscanf::sscanf;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input/samples/6")?;
    let reader = BufReader::new(file);

    println!("Part 1: {}", 0);
    println!("Part 2: {}", 0);

    Ok(())
}
