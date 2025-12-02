use sscanf::sscanf;
use std::fs::read_to_string;

fn read_input(name: &str) -> std::io::Result<Vec<(u64, u64)>> {
    Ok(read_to_string(name)?
        .trim_end()
        .split(',')
        .map(|r| sscanf!(r, "{u64}-{u64}").unwrap())
        .collect())
}

fn is_repeated_twice(n: u64) -> bool {
    let n_length = n.ilog10() + 1;
    if n_length % 2 != 0 {
        return false;
    }

    let power = (10 as u64).pow(n_length/2);
    return (n/power) == (n % power);
}

fn main() -> std::io::Result<()> {
    let mut part1: u64 = 0;
    for (from, to) in read_input("input/2")? {
        for n in from..(to + 1) {
            if is_repeated_twice(n) {
                part1 += n;
            }
        }
    }
    println!("Part1: {part1}");
    Ok(())
}
