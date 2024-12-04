#[allow(dead_code)] // not yet used in template
const INPUT: &str = include_str!("../../inputs/input04.txt");

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

pub fn part1() -> u32 {
    let chars: Vec<Vec<char>> = INPUT.lines().map(|l| l.chars().collect()).collect();
    let word = ['X', 'M', 'A', 'S'];
    let max_x = chars[0].len() as i32 - 1;
    let max_y = chars.len() as i32 - 1;
    let mut count = 0;
    for x in 0..chars[0].len() as i32 {
        for y in 0..chars.len() as i32 {
            'dir: for (x_d, y_d) in DIRECTIONS {
                for pos in 0..word.len() as i32 {
                    let posx = x + pos * x_d;
                    let posy = y + pos * y_d;
                    if posx < 0 || posx > max_x || posy < 0 || posy > max_y {
                        continue 'dir;
                    }
                    if chars[posx as usize][posy as usize] != word[pos as usize] {
                        continue 'dir;
                    }
                }
                count += 1;
                // println!("XMAS found at ({},{}) direction ({},{})", x, y, x_d, y_d);
            }
        }
    }
    count
}

pub fn part2() -> u32 {
    let chars: Vec<Vec<char>> = INPUT.lines().map(|l| l.chars().collect()).collect();
    let word_pairs = [
        (['M', 'A', 'S'], ['M', 'A', 'S']),
        (['M', 'A', 'S'], ['S', 'A', 'M']),
        (['S', 'A', 'M'], ['M', 'A', 'S']),
        (['S', 'A', 'M'], ['S', 'A', 'M']),
    ];
    let mut count = 0;
    for x in 0..(chars[0].len() - 2) {
        for y in 0..(chars.len() - 2) {
            'pair: for (word1, word2) in word_pairs {
                for pos in 0..3 {
                    if chars[x + pos][y + pos] != word1[pos] {
                        continue 'pair;
                    }
                    if chars[x + pos][y + (2 - pos)] != word2[pos] {
                        continue 'pair;
                    }
                }
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 2639);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 2005);
    }
}
