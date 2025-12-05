use sscanf::sscanf;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn contains(s: (u64, u64), p: u64) -> bool {
    p >= s.0 && p <= s.1
}

fn is_intersecting(s1: (u64, u64), s2: (u64, u64)) -> bool {
    contains(s1, s2.0) || contains(s2, s1.0)
}

fn intersect(s1: (u64, u64), s2: (u64, u64)) -> (u64, u64) {
    if contains(s1, s2.0) {
        (s1.0, s1.1.max(s2.1))
    } else {
        intersect(s2, s1)
    }
}

fn add(db: &mut Vec<(u64, u64)>, mut range: (u64, u64)) {
    'main: loop {
        for i in 0..db.len() {
            if is_intersecting(db[i], range) {
                range = intersect(db.swap_remove(i), range);
                continue 'main;
            }
        }
        break;
    }

    db.push(range);
}

fn test_is_intersecting() {
    for (a, b, c, d) in [(1, 5, 5, 7), (1, 5, 2, 4), (3, 7, 1, 3), (3, 7, 1, 4)] {
        assert!(is_intersecting((a, b), (c, d)));
        assert!(is_intersecting((c, d), (a, b)));
    }

    for (a, b, c, d) in [(1, 5, 6, 7)] {
        assert!(!is_intersecting((a, b), (c, d)));
        assert!(!is_intersecting((c, d), (a, b)));
    }
}

fn test_intersect() {
    for (a, b, r) in [((1, 5), (5, 7), (1, 7)), ((1, 5), (2, 3), (1, 5))] {
        let v = intersect(a, b);
        assert!(v == r, "{:?} != {:?}", v, r);
        assert!(intersect(a, b) == intersect(b, a));
    }
}

fn test_add() {
    let mut db: Vec<(u64, u64)> = Vec::new();
    add(&mut db, (2, 3));
    add(&mut db, (4, 5));
    add(&mut db, (1, 8));
    add(&mut db, (15, 18));
}

fn main() -> std::io::Result<()> {
    test_is_intersecting();
    test_intersect();
    test_add();

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
        add(&mut db, (a, b));
    }

    let fresh = lines
        .map(|l| l.unwrap())
        .map(|l| l.parse::<u64>().unwrap())
        .filter(|&l| db.iter().any(|&s| contains(s, l)))
        .count();

    let total: u64 = db.iter().map(|&(a, b)| b - a + 1).sum();

    println!("Part 1: {fresh}");
    println!("Part 2: {total}");

    Ok(())
}
