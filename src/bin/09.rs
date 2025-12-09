use itertools::Itertools;
use sscanf::sscanf;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Point = (i64, i64);
type Segment = (i64, i64);

fn contains(l: &Vec<Segment>, r: &Segment) -> bool {
    l.iter()
        .any(|&s| (s.0 <= r.0 && s.1 >= r.0) && (s.0 <= r.1 && s.1 >= r.1))
}

fn area(p1: &Point, p2: &Point) -> i64 {
    ((p1.0 - p2.0).abs() + 1) * ((p1.1 - p2.1).abs() + 1)
}

struct Figure {
    sweeps: BTreeMap<i64, Vec<Segment>>,
}

enum Pos {
    Left,
    Right,
    In,
}
use Pos::*;

fn pos(x: i64, s: &Segment) -> Pos {
    if x < s.0 {
        Left
    } else if x >= s.0 && x <= s.1 {
        In
    } else {
        Right
    }
}

fn s_xor(line: &Vec<Segment>, s: &Segment) -> Vec<Segment> {
    // println!("{line:?} xor {s:?}");
    if line.is_empty() {
        return vec![*s];
    }

    let mut result = Vec::<Segment>::new();
    let mut line_iter = line.iter();
    line_iter
        .by_ref()
        .take_while_ref(|&ls| matches!(pos(s.0, ls), Right) && matches!(pos(s.1, ls), Right))
        .for_each(|&ls| result.push(ls));

    let next = line_iter.next();
    // println!("after rr => {result:?}, {next:?}");
    match next {
        None => {
            result.push(*s);
        }
        Some(ls) => match (pos(s.0, ls), pos(s.1, ls)) {
            (Left, Left) => {
                result.push(*s);
                result.push(*ls);
            }
            (Left, In) => result.push((s.0, ls.1)),
            (In, Right) => result.push((ls.0, s.1)),

            (In, In) => {
                if s == ls {
                } else if s.0 == ls.0 {
                    result.push((s.1, ls.1));
                } else if s.1 == ls.1 {
                    result.push((ls.0, s.0));
                } else {
                    result.push((ls.0, s.0));
                    result.push((s.1, ls.1));
                }
            }

            _ => unreachable!(),
        },
    }

    let next = line_iter.next();
    // println!("after => {result:?}, {next:?}");
    match next {
        None => {}
        Some(ls) => {
            if ls.0 == s.1 {
                let prev = result.pop().unwrap();
                result.push((prev.0, ls.1));
            } else {
                result.push(*ls);
            }
        }
    }

    line_iter.for_each(|&ls| result.push(ls));
    result
}

fn s_or(line: &Vec<Segment>, s: &Segment) -> Vec<Segment> {
    // println!("{line:?} xor {s:?}");
    if line.is_empty() {
        return vec![*s];
    }

    let mut result = Vec::<Segment>::new();
    let mut line_iter = line.iter();
    line_iter
        .by_ref()
        .take_while_ref(|&ls| matches!(pos(s.0, ls), Right) && matches!(pos(s.1, ls), Right))
        .for_each(|&ls| result.push(ls));

    let next = line_iter.next();
    // println!("after rr => {result:?}, {next:?}");
    match next {
        None => {
            result.push(*s);
        }
        Some(ls) => match (pos(s.0, ls), pos(s.1, ls)) {
            (Left, Left) => {
                result.push(*s);
                result.push(*ls);
            }
            (Left, In) => result.push((s.0, ls.1)),
            (In, Right) => result.push((ls.0, s.1)),
            (In, In) => result.push(*ls),
            _ => unreachable!(),
        },
    }

    let next = line_iter.next();
    // println!("after => {result:?}, {next:?}");
    match next {
        None => {}
        Some(ls) => {
            if ls.0 == s.1 {
                let prev = result.pop().unwrap();
                result.push((prev.0, ls.1));
            } else {
                result.push(*ls);
            }
        }
    }

    line_iter.for_each(|&ls| result.push(ls));
    result
}

impl Figure {
    fn new(segments: BTreeMap<i64, Vec<Segment>>) -> Self {
        let mut scanline = Vec::<Segment>::new();
        let mut y_s: i64 = 0;
        let mut result = BTreeMap::<i64, Vec<Segment>>::new();

        for (y, segment_list) in segments {
            // println!("{y} {segment_list:?} {scanline:?}");
            let mut next = scanline.clone();
            let mut at = scanline.clone();
            for segment in segment_list {
                next = s_xor(&next, &segment);
                at = s_or(&at, &segment);
            }

            if y_s != y {
                result.insert(y_s, scanline);
            }
            result.insert(y, at);

            y_s = y + 1;
            scanline = next;
        }

        Self { sweeps: result }
    }

    fn contains_rect(&self, a: &Point, b: &Point) -> bool {
        let x = (a.0.min(b.0), a.0.max(b.0));
        let y = a.1.min(b.1)..=a.1.max(b.1);
        self.sweeps.range(y).all(|(_, s)| contains(s, &x))
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input/9")?;
    let mut visual = File::create("output/9.svg")?;

    let points: Vec<Point> = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|l| sscanf!(l, "{i64},{i64}"))
        .filter_map(Result::ok)
        .collect();

    let max_x = points.iter().map(|p| p.0).max().unwrap();
    let max_y = points.iter().map(|p| p.1).max().unwrap();

    visual.write_fmt(format_args!(
        "<svg width='{max_x}' height='{max_y}' xmlns='http://www.w3.org/2000/svg'>
<path d='M {} {}",
        points[0].0, points[0].1
    ));
    for &(x, y) in points.iter() {
        visual.write_fmt(format_args!("L {x} {y} "));
    }
    visual.write_all(
        b"Z' fill='transparent' stroke='black' stroke-width='10' style='fill:none' />\n",
    );

    let part1 = (0..points.len())
        .tuple_combinations()
        .map(|(a, b)| area(&points[a], &points[b]))
        .max()
        .unwrap();

    let mut segments: BTreeMap<i64, Vec<Segment>> = BTreeMap::new();
    (0..points.len())
        .map(|i| (&points[i], &points[(i + 1) % points.len()]))
        .filter(|(a, b)| a.1 == b.1)
        .for_each(|(a, b)| {
            let p = (a.0.min(b.0), a.0.max(b.0));
            match segments.get_mut(&a.1) {
                Some(s) => {
                    s.push(p);
                }
                None => {
                    segments.insert(a.1, vec![p]);
                }
            }
        });

    let figure = Figure::new(segments);

    for (y, line) in figure.sweeps.iter() {
        for (x1, x2) in line {
            visual.write_fmt(format_args!(
                "<line x1='{x1}' x2='{x2}' y1='{y}' y2='{y}' style='stroke:red;stroke-width:10'/>\n"
            ));
        }
    }

    let part2_ab = (0..points.len())
        .tuple_combinations()
        .map(|(a, b)| (&points[a], &points[b]))
        .sorted_by_key(|&(a, b)| area(a, b))
        .rev()
        .find(|&(a, b)| figure.contains_rect(a, b))
        .unwrap();

    let part2 = area(part2_ab.0, part2_ab.1);

    println!("Part 1: {part1}");
    println!("Part 2: {part2} {part2_ab:?}");

    visual.write_fmt(format_args!(
        "<circle cx='{}' cy='{}' r='150' />\n",
        part2_ab.0 .0, part2_ab.0 .1
    ));
    visual.write_fmt(format_args!(
        "<circle cx='{}' cy='{}' r='150' />\n",
        part2_ab.1 .0, part2_ab.1 .1
    ));
    visual.write_all(b"</xml>");
    visual.flush();
    Ok(())
}
