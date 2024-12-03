use regex::Regex;
const INPUT: &str = include_str!("../../inputs/input03.txt");

pub fn part1() -> i64 {
    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let re = Regex::new(pattern).unwrap();
    re.captures_iter(INPUT)
        .map(|cap| {
            cap.get(1).unwrap().as_str().parse::<i64>().unwrap()
                * cap.get(2).unwrap().as_str().parse::<i64>().unwrap()
        })
        .sum()
}

pub fn part2() -> i64 {
    let mut pos: usize = 0;
    let mut sum: i64 = 0;
    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let re = Regex::new(pattern).unwrap();
    while pos < INPUT.len() {
        match INPUT[pos..].find("don't()") {
            Some(foundpos) => {
                let end = pos + foundpos;
                sum += re
                    .captures_iter(&INPUT[pos..end])
                    .map(|cap| {
                        cap.get(1).unwrap().as_str().parse::<i64>().unwrap()
                            * cap.get(2).unwrap().as_str().parse::<i64>().unwrap()
                    })
                    .sum::<i64>();
                match INPUT[end..].find("do()") {
                    Some(founddo) => {
                        pos = end + founddo;
                    }
                    None => {
                        pos = INPUT.len();
                    }
                }
            }
            None => {
                sum += re
                    .captures_iter(&INPUT[pos..])
                    .map(|cap| {
                        cap.get(1).unwrap().as_str().parse::<i64>().unwrap()
                            * cap.get(2).unwrap().as_str().parse::<i64>().unwrap()
                    })
                    .sum::<i64>();
                pos = INPUT.len();
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 191183308);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 92082041);
    }
}
