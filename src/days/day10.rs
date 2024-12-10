#[allow(dead_code)] // not yet used in template
const INPUT: &str = include_str!("../../inputs/input10.txt");

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn part1() -> usize {
    let map: Vec<&[u8]> = INPUT.lines().map(|l| l.as_bytes()).collect();
    let max_x = map[0].len();
    let max_y = map.len();
    let mut count: usize = 0;
    // positions we have already visited
    let mut v_map: Vec<Vec<bool>> = vec![vec![false; max_x]; max_y];
    let mut options: Vec<(usize, usize)> = Vec::new();
    let mut new_options: Vec<(usize, usize)> = Vec::new();
    for x in 0..max_x {
        for y in 0..max_y {
            if map[x][y] == b'0' {
                // println!("trailhead ({},{})", x, y);
                // reset visibility map
                for row in &mut v_map {
                    for value in row {
                        *value = false;
                    }
                }
                options.clear();
                options.push((x, y));
                // find a path
                for i in 1..10 {
                    // println!("  options for {}: {:?}", i, options);
                    for o in &options {
                        for d in DIRECTIONS {
                            let (pos_x, pos_y) = (o.0 as i32 + d.0, o.1 as i32 + d.1);
                            if pos_x >= 0
                                && pos_y >= 0
                                && pos_x < max_x as i32
                                && pos_y < max_y as i32
                                && map[pos_x as usize][pos_y as usize] == b'0' + i
                                && !v_map[pos_x as usize][pos_y as usize]
                            {
                                v_map[pos_x as usize][pos_y as usize] = true;
                                new_options.push((pos_x as usize, pos_y as usize));
                                if i == 9 {
                                    // println!("    ends at ({},{})", pos_x, pos_y);
                                    count += 1;
                                }
                            }
                        }
                    }
                    options.clear();
                    options = new_options.clone();
                    new_options.clear();
                }
            }
        }
    }

    count
}

pub fn part2() -> usize {
    let map: Vec<&[u8]> = INPUT.lines().map(|l| l.as_bytes()).collect();
    let max_x = map[0].len();
    let max_y = map.len();
    let mut count: usize = 0;
    // positions we have already visited
    let mut v_map: Vec<Vec<bool>> = vec![vec![false; max_x]; max_y];
    // in how many ways can we reach the 9 from a position
    let mut r_map: Vec<Vec<usize>> = vec![vec![0 as usize; max_x]; max_y];
    for target in (1..10).rev() {
        for x in 0..max_x {
            for y in 0..max_y {
                if map[x][y] == b'0' + target - 1 {
                    // we start searching for 8->9 then 7-> 8...
                    // println!("trailhead ({},{})", x, y);
                    // reset visibility map
                    for row in &mut v_map {
                        for value in row {
                            *value = false;
                        }
                    }

                    // find a path to target

                    // println!("  options for {}: {:?}", i, options);
                    let mut r: usize = 0;
                    for d in DIRECTIONS {
                        let (pos_x, pos_y) = (x as i32 + d.0, y as i32 + d.1);
                        if pos_x >= 0
                            && pos_y >= 0
                            && pos_x < max_x as i32
                            && pos_y < max_y as i32
                            && map[pos_x as usize][pos_y as usize] == b'0' + target
                            && !v_map[pos_x as usize][pos_y as usize]
                        {
                            v_map[pos_x as usize][pos_y as usize] = true;
                            if target == 9 {
                                r += 1;
                            } else {
                                r += r_map[pos_x as usize][pos_y as usize]
                            }
                        }
                    }
                    r_map[x][y] = r; // in how many ways can we reach 9 from [x,y]
                    if target == 1 {
                        count += r;
                    }
                }
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
        assert_eq!(part1(), 688);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 1459);
    }
}
