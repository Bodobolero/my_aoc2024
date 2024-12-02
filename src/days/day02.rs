use itertools::Itertools;

#[allow(dead_code)] // not yet used in template
const INPUT: &str = include_str!("../../inputs/input02.txt");

fn increasing<I>(mut items: I) -> bool
where
    I: Iterator<Item = i16>,
    I: std::fmt::Debug,
{
    let increasing = items.try_fold(None, |prev, current| match prev {
        Some(prev) if current > prev && (current - prev) <= 3 => Ok(Some(current)),
        None => Ok(Some(current)),
        _ => Err("not increasing"),
    });

    increasing.ok().is_some()
}

fn decreasing<I>(mut items: I) -> bool
where
    I: Iterator<Item = i16>,
    I: std::fmt::Debug,
{
    let decreasing = items.try_fold(None, |acc, current| match acc {
        Some(prev) if current < prev && (prev - current) <= 3 => Ok(Some(current)),
        None => Ok(Some(current)),
        _ => Err("not decreasing"),
    });
    decreasing.ok().is_some()
}

pub fn part1() -> usize {
    INPUT
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse::<i16>().unwrap())
        })
        .filter(|items| decreasing(items.clone()) || increasing(items.clone()))
        .count()
}

fn increasing_with_dampener<I>(items: I) -> bool
where
    I: Iterator<Item = i16>,
    I: std::fmt::Debug,
    I: Clone,
{
    let combinations = items.clone().combinations(items.count() - 1);
    for damp_items in combinations {
        let increasing = damp_items
            .iter()
            .try_fold(None, |prev, current| match prev {
                Some(prev) if current > prev && (current - prev) <= 3 => Ok(Some(current)),
                None => Ok(Some(current)),
                _ => Err("not increasing"),
            });
        if increasing.ok().is_some() {
            return true;
        }
    }
    false
}

fn decreasing_with_dampener<I>(items: I) -> bool
where
    I: Iterator<Item = i16>,
    I: std::fmt::Debug,
    I: Clone,
{
    let combinations = items.clone().combinations(items.count() - 1);
    for damp_items in combinations {
        let decreasing = damp_items.iter().try_fold(None, |acc, current| match acc {
            Some(prev) if current < prev && (prev - current) <= 3 => Ok(Some(current)),
            None => Ok(Some(current)),
            _ => Err("not decreasing"),
        });
        if decreasing.ok().is_some() {
            return true;
        }
    }
    false
}

pub fn part2() -> usize {
    INPUT
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse::<i16>().unwrap())
        })
        .filter(|items| {
            decreasing_with_dampener(items.clone()) || increasing_with_dampener(items.clone())
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 680);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 710);
    }
}
