use itertools::Itertools;
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

    fn _path_count(&self, from: usize, to: usize) -> usize {
        if from == to {
            return 1;
        }

        self.edges[from]
            .iter()
            .map(|&v| self._path_count(v, to))
            .sum()
    }

    fn path_count(&self, from: &str, to: &str) -> usize {
        return self._path_count(
            *self
                .vertices
                .get(&from.to_string())
                .expect("from must exist"),
            *self.vertices.get(&to.to_string()).expect("to must exit"),
        );
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

    let part1 = graph.path_count("you", "out");
    println!("Part 1: {part1}");

    Ok(())
}
