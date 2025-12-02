use sscanf::sscanf;
use std::fs::read_to_string;

fn read_input(name: &str) -> std::io::Result<Vec<(u64, u64)>> {
    Ok(read_to_string(name)?
        .trim_end()
        .split(',')
        .map(|r| sscanf!(r, "{u64}-{u64}").unwrap())
        .collect())
}

fn digit_count(n: u64) -> u32 {
    n.ilog10() + 1
}

fn is_repeated_x_times(nn: u64, x: u32) -> bool {
    let mut n = nn;
    let len = digit_count(n);
    if len % x != 0 {
        return false;
    }

    let power = (10 as u64).pow(len / x);
    let pattern = n % power;
    n /= power;
    while n > 0 {
        if n % power != pattern {
            return false;
        }
        n /= power;
    }

    return true;
}

fn test_is_repeated_x_times() {
    assert!(is_repeated_x_times(11, 2));
    assert!(is_repeated_x_times(999, 3));
    assert!(is_repeated_x_times(1188511885, 2));
    assert!(is_repeated_x_times(565656, 3));

    assert!(!is_repeated_x_times(100, 2));
}

fn main() -> std::io::Result<()> {
    test_is_repeated_x_times();

    let mut part1: u64 = 0;
    let mut part2: u64 = 0;
    for (from, to) in read_input("input/2")? {
        for n in from..=to {
            for t in 2..=digit_count(n) {
                if is_repeated_x_times(n, t) {
                    if t == 2 {
                        part1 += n;
                    }

                    part2 += n;
                    break;
                }
            }
        }
    }
    println!("Part1: {part1}");
    println!("Part2: {part2}");
    Ok(())
}
