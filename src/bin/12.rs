use itertools::Itertools;
use sscanf::sscanf;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Task {
    w: usize,
    h: usize,
    cnt: Vec<usize>,
}

impl Task {
    fn new(s: &String) -> Self {
        let (size, counts) = s.split(": ").collect_tuple().unwrap();
        let (w, h) = sscanf!(size, "{usize}x{usize}").unwrap();
        let cnt = counts.split(' ').map(|c| c.parse().unwrap()).collect_vec();
        Self { w, h, cnt }
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input/12")?;

    let mut in_shape = false;
    let mut tasks = Vec::<Task>::new();
    let mut shape: u32 = 0;
    let mut shapes = Vec::<u32>::new();
    for l in BufReader::new(file).lines().filter_map(Result::ok) {
        if l.ends_with(':') {
            in_shape = true;
        } else if l.is_empty() {
            in_shape = false;
            shapes.push(shape);
            shape = 0;
        } else if in_shape {
            shape = shape * 8
                + l.chars()
                    .fold(0, |acc, b| acc * 2 + if b == '#' { 1 } else { 0 });
        } else {
            tasks.push(Task::new(&l));
        }
    }

    let mut fit = 0;
    for t in tasks {
        let a = t.w * t.h;
        let b = t
            .cnt
            .iter()
            .enumerate()
            .map(|(i, &c)| c * (shapes[i].count_ones() as usize))
            .sum::<usize>();

        let diff = (a as i64) - (b as i64);
        if diff > 0 {
            fit += 1;
        }
    }

    // ¯\\_(ツ)_/¯
    println!("total: {fit}");

    Ok(())
}
