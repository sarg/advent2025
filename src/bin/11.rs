use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Graph {
    vertices: BTreeMap<String, usize>,
    edges: Vec<Vec<usize>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            vertices: BTreeMap::new(),
            edges: Vec::new(),
        }
    }

    fn idx(&mut self, v: String) -> usize {
        match self.vertices.get(&v) {
            None => {
                let idx = self.vertices.len();
                self.vertices.insert(v, idx);
                self.edges.push(vec![]);
                idx
            }
            Some(&v) => v,
        }
    }

    fn add_edge(&mut self, from: String, to: String) {
        let f = self.idx(from);
        let t = self.idx(to);
        self.edges[f].push(t);
    }

    fn _path_count(&self, from: usize, to: usize, cache: &mut BTreeMap<usize, usize>) -> usize {
        if let Some(&n) = cache.get(&from) {
            return n;
        }

        let count = if from == to {
            1
        } else {
            self.edges[from]
                .iter()
                .map(|&v| self._path_count(v, to, cache))
                .sum()
        };

        cache.insert(from, count);
        count
    }

    fn _idx(&self, v: &str) -> usize {
        *self.vertices.get(&v.to_string()).expect("must exist")
    }

    fn path_count(&self, path: &[&str]) -> usize {
        path.iter()
            .zip(path.iter().skip(1))
            .map(|(a, b)| self._path_count(self._idx(a), self._idx(b), &mut BTreeMap::new()))
            .product()
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input/11")?;
    let mut graph = Graph::new();

    let reader = BufReader::new(file);
    for l in reader.lines() {
        let line = l?;

        let mut liter = line.split(' ');
        let mut from = liter.next().unwrap();
        from = &from[0..from.len() - 1];

        for to in liter {
            graph.add_edge(from.to_string(), to.to_string());
        }
    }

    let part1 = graph.path_count(&["you", "out"]);
    println!("Part 1: {part1}");

    let part2 = graph.path_count(&["svr", "dac", "fft", "out"])
        + graph.path_count(&["svr", "fft", "dac", "out"]);

    println!("Part 2: {part2}");

    Ok(())
}
