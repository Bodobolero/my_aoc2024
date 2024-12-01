use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/input01.txt");

pub fn part1() -> i64 {
    let (mut a, mut b): (Vec<_>, Vec<_>) = INPUT
        .lines()
        .map(|l| {
            let mut split = l.split_whitespace();
            (split.next().unwrap(), split.next().unwrap())
        })
        .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
        .unzip();
    a.sort_unstable();
    b.sort_unstable();
    a.iter().zip(b.iter()).map(|(a, b)| (a - b).abs()).sum()
}

pub fn part2() -> i64 {
    let (a, b): (Vec<_>, Vec<_>) = INPUT
        .lines()
        .map(|l| {
            let mut split = l.split_whitespace();
            (split.next().unwrap(), split.next().unwrap())
        })
        .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
        .unzip();
    let mut counts = HashMap::<i64, i64>::new();
    for item in b {
        let count = counts.entry(item).or_insert(0);
        *count += 1;
    }
    a.iter()
        .map(|val| counts.get(val).unwrap_or(&0) * val)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 3246517);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 29379307);
    }
}
