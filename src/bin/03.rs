use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn joltage_n(cells: &Vec<u32>, len: usize) -> u64 {
    // greedy algo: select max digit from the slice of possible locations for ith digit
    // ith digit goes after the previous, leaving enough space for (len-i) digits at the end
    let mut bat = vec![0; len];
    let mut l = 0;
    let mut r = cells.len() - len;
    for i in 0..len {
        let mut max_i = l;
        for mi in l..=r {
            if cells[mi] > cells[max_i] {
                max_i = mi;
            }
        }

        bat[i] = cells[max_i];
        l = max_i + 1;
        r += 1;
    }

    bat.iter().fold(0, |e, &v| e * 10 + (v as u64))
}

fn main() -> std::io::Result<()> {
    let file = File::open("input/3")?;
    let reader = BufReader::new(file);

    let mut part1 = 0;
    let mut part2 = 0;
    for line in reader.lines() {
        let line_content = line?;
        let cells: Vec<u32> = line_content
            .chars()
            .map(|c| c.to_digit(10).expect("digit"))
            .collect();

        part1 += joltage_n(&cells, 2);
        part2 += joltage_n(&cells, 12);
    }

    println!("Part1: {part1}");
    println!("Part2: {part2}");
    Ok(())
}
