use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/input08.txt");

pub fn part1() -> usize {
    let mut antennas: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
    let mut max_x: i64 = 0;
    let mut max_y: i64 = 0;
    for (i, line) in INPUT.lines().enumerate() {
        max_x = i as i64;
        max_y = line.as_bytes().len() as i64;
        for (j, roof) in line.as_bytes().iter().enumerate() {
            if *roof != b'.' {
                antennas.entry(*roof).or_default().push((i, j));
            }
        }
    }
    max_x += 1;
    // println!("anntennas: {:?}", antennas);
    let mut points: usize = 0;
    // mark unique roofs with resonation points
    let mut roofs: Vec<Vec<u8>> = vec![vec![b' '; max_y as usize]; max_x as usize];
    for (_letter, positions) in &antennas {
        for combo in positions.iter().combinations(2) {
            // println!("  letter {} combo {:?}", letter, combo);
            // mark resonation points
            let r1_x = combo[0].0 as i64;
            let r1_y = combo[0].1 as i64;
            let r2_x = combo[1].0 as i64;
            let r2_y = combo[1].1 as i64;
            let (p1_x, p1_y) = (r2_x + (r2_x - r1_x), r2_y + (r2_y - r1_y));
            if p1_x >= 0
                && p1_y >= 0
                && p1_x < max_x
                && p1_y < max_y
                && roofs[p1_x as usize][p1_y as usize] != b'#'
            {
                points += 1;
                roofs[p1_x as usize][p1_y as usize] = b'#';
                // println!("     point ({}.{})", p1_x, p1_y);
            }
            let (p1_x, p1_y) = (r1_x - (r2_x - r1_x), r1_y - (r2_y - r1_y));
            if p1_x >= 0
                && p1_y >= 0
                && p1_x < max_x
                && p1_y < max_y
                && roofs[p1_x as usize][p1_y as usize] != b'#'
            {
                points += 1;
                roofs[p1_x as usize][p1_y as usize] = b'#';
                // println!("     point ({}.{})", p1_x, p1_y);
            }
        }
    }
    points
}

pub fn part2() -> usize {
    let mut antennas: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
    let mut max_x: i64 = 0;
    let mut max_y: i64 = 0;
    for (i, line) in INPUT.lines().enumerate() {
        max_x = i as i64;
        max_y = line.as_bytes().len() as i64;
        for (j, roof) in line.as_bytes().iter().enumerate() {
            if *roof != b'.' {
                antennas.entry(*roof).or_default().push((i, j));
            }
        }
    }
    max_x += 1;
    // println!("anntennas: {:?}", antennas);
    let mut points: usize = 0;
    // mark unique roofs with resonation points
    let mut roofs: Vec<Vec<u8>> = vec![vec![b' '; max_y as usize]; max_x as usize];
    for (_letter, positions) in &antennas {
        for combo in positions.iter().combinations(2) {
            // println!("  letter {} combo {:?}", letter, combo);
            // mark resonation points
            let r1_x = combo[0].0 as i64;
            let r1_y = combo[0].1 as i64;
            let r2_x = combo[1].0 as i64;
            let r2_y = combo[1].1 as i64;

            let (mut p1_x, mut p1_y) = (r1_x, r1_y);
            let mut dist = 0i64;
            while p1_x >= 0 && p1_y >= 0 && p1_x < max_x && p1_y < max_y {
                if roofs[p1_x as usize][p1_y as usize] != b'#' {
                    points += 1;
                    roofs[p1_x as usize][p1_y as usize] = b'#';
                }
                dist += 1;
                (p1_x, p1_y) = (r2_x + dist * (r2_x - r1_x), r2_y + dist * (r2_y - r1_y));
            }
            let (mut p1_x, mut p1_y) = (r2_x, r2_y);
            let mut dist = 0i64;
            while p1_x >= 0 && p1_y >= 0 && p1_x < max_x && p1_y < max_y {
                if roofs[p1_x as usize][p1_y as usize] != b'#' {
                    points += 1;
                    roofs[p1_x as usize][p1_y as usize] = b'#';
                }
                dist += 1;
                (p1_x, p1_y) = (r1_x - dist * (r2_x - r1_x), r1_y - dist * (r2_y - r1_y));
            }
        }
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 364);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 1231);
    }
}
