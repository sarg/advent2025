use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Graph<'a> = BTreeMap<&'a str, Vec<&'a str>>;

fn _path_count<'a>(
    g: &'a Graph,
    from: &'a str,
    to: &'a str,
    cache: &mut BTreeMap<&'a str, usize>,
) -> usize {
    if from == to {
        1
    } else if let Some(&n) = cache.get(from) {
        n
    } else if let Some(adjacent) = g.get(from) {
        let n = adjacent.iter().map(|v| _path_count(g, v, to, cache)).sum();
        cache.insert(from, n);
        n
    } else {
        0
    }
}

fn path_count(g: &Graph, path: &[&str]) -> usize {
    path.iter()
        .zip(path.iter().skip(1))
        .map(|(&a, &b)| _path_count(g, a, b, &mut BTreeMap::new()))
        .product()
}

fn main() -> std::io::Result<()> {
    let file = File::open("input/11")?;
    let mut graph = Graph::new();

    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect();

    for line in lines.iter() {
        let mut liter = line.split(' ');
        let mut from = liter.next().unwrap();
        from = &from[0..from.len() - 1];
        graph.insert(from, liter.collect());
    }

    let part1 = path_count(&graph, &["you", "out"]);
    println!("Part 1: {part1}");

    let part2: usize = [["svr", "dac", "fft", "out"], ["svr", "fft", "dac", "out"]]
        .iter()
        .map(|p| path_count(&graph, p))
        .sum();

    println!("Part 2: {part2}");

    Ok(())
}
