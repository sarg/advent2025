use sscanf::sscanf;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn contains(db: &Vec<(u64, u64)>, id: u64) -> bool {
    for (a, b) in db {
        if id >= *a && id <= *b {
            return true;
        }
    }
    return false;
}

fn main() -> std::io::Result<()> {
    let file = File::open("input/5")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let mut db: Vec<(u64, u64)> = Vec::new();
    loop {
        let l = lines.next().expect("line").expect("");
        if l.is_empty() {
            break;
        }

        let (a, b) = sscanf!(l, "{u64}-{u64}").unwrap();
        db.push((a, b));
    }

    let fresh = lines
        .map(|l| l.unwrap())
        .map(|l| l.parse::<u64>().unwrap())
        .filter(|l| contains(&db, *l))
        .count();

    println!("Part 1: {fresh}");

    Ok(())
}
