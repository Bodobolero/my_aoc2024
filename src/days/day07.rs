const INPUT: &str = include_str!("../../inputs/input07.txt");

const fn add(a: u64, b: u64) -> u64 {
    a + b
}

const fn mult(a: u64, b: u64) -> u64 {
    a * b
}

const fn concat(a: u64, b: u64) -> u64 {
    // format!("{}{}", a, b).parse::<u64>().unwrap() -> this is not const
    let mut multiplier = 1;
    let mut temp = b;

    // Determine the magnitude of `b` (how many digits it has)
    // Todo: test if Log base 10 is faster
    while temp > 0 {
        multiplier *= 10;
        temp /= 10;
    }

    // Concatenate by shifting `a` and adding `b`
    a * multiplier + b
}

const OPERATIONS: [fn(u64, u64) -> u64; 2] = [add, mult];

const OPERATIONS_PART2: [fn(u64, u64) -> u64; 3] = [add, mult, concat];

fn solve_equation(
    equation: u64,
    operands: &[u64],
    operations: &[fn(u64, u64) -> u64],
) -> Option<u64> {
    let mut many_results: Vec<u64> = vec![operands[0]];
    for op2 in &operands[1..] {
        let mut new_results: Vec<u64> = Vec::new();
        for op1 in many_results {
            for operation in operations {
                let res = operation(op1, *op2);
                if res <= equation {
                    new_results.push(res);
                }
            }
        }
        many_results = new_results;
    }
    many_results.into_iter().find(|&result| result == equation)
}

fn solve(operations: &[fn(u64, u64) -> u64]) -> u64 {
    INPUT
        .lines()
        .map(|l| {
            let mut s = l.split(':');
            let val = s.next().unwrap().parse::<u64>().unwrap();
            let operators: Vec<u64> = s
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|ostr| ostr.parse::<u64>().unwrap())
                .collect();

            (val, operators)
        })
        .filter_map(|(e, ops)| solve_equation(e, &ops, operations))
        .sum()
}

pub fn part1() -> u64 {
    solve(&OPERATIONS)
}

pub fn part2() -> u64 {
    solve(&OPERATIONS_PART2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 4998764814652);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 37598910447546);
    }
}
