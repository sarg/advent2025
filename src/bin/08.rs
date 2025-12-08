use itertools::Itertools;
use sscanf::sscanf;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Point = (i64, i64, i64);

struct UnionFind {
    roots: Vec<usize>,
    sizes: Vec<usize>,
    n: usize,
}

impl UnionFind {
    fn new(len: usize) -> Self {
        Self {
            roots: (0..len).collect(),
            sizes: vec![1; len],
            n: len,
        }
    }

    fn find(&mut self, v: usize) -> usize {
        if self.roots[v] != v {
            self.roots[v] = self.find(self.roots[v]);
        }
        self.roots[v]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let (r1, r2) = (self.find(a), self.find(b));
        if r1 == r2 {
            return false;
        }

        self.sizes[r1] += self.sizes[r2];
        self.sizes[r2] = 0;
        self.roots[r2] = r1;
        self.n -= 1;
        return true;
    }
}

fn dist(p1: &Point, p2: &Point) -> i64 {
    (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2) + (p1.2 - p2.2).pow(2)
}

fn main() -> std::io::Result<()> {
    let file = File::open("input/8")?;
    let points: Vec<Point> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|l| sscanf!(l, "{i64},{i64},{i64}"))
        .filter_map(Result::ok)
        .collect();

    let mut pairs = (0..points.len())
        .tuple_combinations()
        .sorted_unstable_by_key(|&(p1, p2)| dist(&points[p1], &points[p2]));

    let mut uf = UnionFind::new(points.len());
    for (a, b) in pairs.by_ref().take(1000) {
        uf.union(a, b);
    }
    let part1 = uf.sizes.iter().sorted().rev().take(3).product::<usize>();

    let last = pairs.find(|&(a, b)| uf.union(a, b) && uf.n == 1).unwrap();
    let part2 = points[last.0].0 * points[last.1].0;
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}
