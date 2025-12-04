use itertools::iproduct;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn find_unblocked(grid: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let w = grid[0].len() as i32;
    let h = grid.len() as i32;

    let mut unblocked = Vec::new();
    for y in 0..h {
        for x in 0..w {
            if grid[y as usize][x as usize] == b'.' {
                continue;
            }
            let mut n = 0;
            for (dx, dy) in iproduct!((-1 as i32)..=1, (-1 as i32)..=1) {
                let xx = (x as i32) + dx;
                let yy = (y as i32) + dy;
                if xx >= 0 && xx < w && yy >= 0 && yy < h {
                    let c = grid[yy as usize][xx as usize];
                    if c == b'@' {
                        n += 1;
                    }
                }
            }
            if n <= 4 {
                unblocked.push((y as usize, x as usize));
            }
        }
    }
    unblocked
}

fn main() -> std::io::Result<()> {
    let file = File::open("input/4")?;
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<u8>> = reader
        .lines()
        .map(Result::unwrap)
        .map(|s| s.into_bytes())
        .collect();

    println!("Part 1: {}", find_unblocked(&grid).len());

    let mut part2 = 0;
    loop {
        let unblocked = find_unblocked(&grid);
        if unblocked.is_empty() {
            break;
        }
        unblocked.iter().for_each(|(y, x)| {
            grid[*y][*x] = b'.';
            part2 += 1;
        });
    }
    println!("Part 2: {}", part2);
    Ok(())
}
