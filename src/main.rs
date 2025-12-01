use sscanf::sscanf;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const CYCLE_LENGTH: i32 = 100;
const INITIAL_POS: u32 = 50;

fn main() -> std::io::Result<()> {
    let file = File::open("input/1")?;
    let reader = BufReader::new(file);

    let mut part1 = 0;
    let mut part2 = 0;
    let mut dial: u32 = INITIAL_POS;
    for line in reader.lines() {
        let line_content = line?;
        let (dir, count) = sscanf!(line_content, "{char}{u32}")
            .expect(&format!("Failed to parse line: '{}'", line_content));

        let dir: i32 = match dir {
            'L' => -1,
            'R' => 1,
            _ => panic!("Wrong direction: {}", line_content),
        };
        let count = count as i32;
        part2 += count / CYCLE_LENGTH; // full turns

        let count = count % CYCLE_LENGTH; // remainder
        let new_dial = dial as i32 + (dir * count);
        if dial != 0 && (new_dial <= 0 || new_dial >= CYCLE_LENGTH) {
            // remainder goes over 0
            part2 += 1;
        }

        dial = ((new_dial + CYCLE_LENGTH) % CYCLE_LENGTH) as u32;
        if dial == 0 {
            part1 += 1;
        }
        //println!("D: {dir:3} C: {count:4} => {dial:4}, {part1}, {part2}")
    }
    println!("Part1: {part1}");
    println!("Part2: {part2}");
    Ok(())
}
