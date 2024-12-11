use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/input11.txt");

fn solve(blinks: i32) -> usize {
    let stones: Vec<u64> = INPUT
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    // occurrences key: num value: count
    let mut o: HashMap<u64, usize> = HashMap::new();
    for stone in stones {
        *o.entry(stone).or_insert(0) += 1;
    }
    for _i in 0..blinks {
        let mut o2: HashMap<u64, usize> = HashMap::new();
        for (&k, &count) in &o {
            if k == 0 {
                *o2.entry(1).or_insert(0) += count;
            } else {
                let s = format!("{}", k);
                if s.len() % 2 == 0 {
                    let left: u64 = s[0..s.len() / 2].parse::<u64>().unwrap();
                    let right: u64 = s[s.len() / 2..].parse::<u64>().unwrap();
                    *o2.entry(left).or_insert(0) += count;
                    *o2.entry(right).or_insert(0) += count;
                } else {
                    *o2.entry(k * 2024).or_insert(0) += count;
                }
            }
        }
        o = o2.clone();
    }
    o.values().sum()
}

pub fn part1() -> usize {
    solve(25)
}

pub fn part2() -> usize {
    solve(75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 213625);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 252442982856820);
    }
}
