const INPUT: &str = include_str!("../../inputs/input06.txt");

const DIRECTIONS: [(i32, i32); 4] = [
    (-1, 0), // up - start here for part 1
    (0, 1),  // right
    (1, 0),  // down
    (0, -1), // left
];

pub fn part1() -> usize {
    let mut map: Vec<Vec<u8>> = INPUT.lines().map(|line| line.as_bytes().to_vec()).collect();
    let mut x: i32 = 0; // rows from top to bottom
    let mut y: i32 = 0; // chars from left to right
    'outer: for (i, row) in map.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == b'^' {
                x = i as i32;
                y = j as i32;
                break 'outer;
            }
        }
    }
    let mut count: usize = 0;
    let max_x = map.len() as i32;
    let max_y = map[0].len() as i32;
    let mut direction = 0;
    while x >= 0 && y >= 0 && x < max_x && y < max_y {
        if map[x as usize][y as usize] != b'X' {
            // count and mark
            map[x as usize][y as usize] = b'X';
            count += 1;
        }
        // check if we need to change direction
        let next_x = x + DIRECTIONS[direction].0;
        let next_y = y + DIRECTIONS[direction].1;
        if next_x >= 0
            && next_y >= 0
            && next_x < max_x
            && next_y < max_y
            && map[next_x as usize][next_y as usize] == b'#'
        {
            // turn
            direction = (direction + 1) % 4;
        } else {
            // walk
            x += DIRECTIONS[direction].0;
            y += DIRECTIONS[direction].1;
        }
    }
    count
}

pub fn part2() -> usize {
    let mut map: Vec<Vec<u8>> = INPUT.lines().map(|line| line.as_bytes().to_vec()).collect();
    let original_map: Vec<Vec<u8>> = map.to_vec();
    let mut x: i32 = 0; // rows from top to bottom
    let mut y: i32 = 0; // chars from left to right
    'outer: for (i, row) in map.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == b'^' {
                x = i as i32;
                y = j as i32;
                break 'outer;
            }
        }
    }
    let start_x = x;
    let start_y = y;
    let max_x = map.len() as i32;
    let max_y = map[0].len() as i32;
    let mut direction = 0;
    while x >= 0 && y >= 0 && x < max_x && y < max_y {
        if map[x as usize][y as usize] != b'X' {
            // count and mark
            map[x as usize][y as usize] = b'X';
        }
        // check if we need to change direction
        let next_x = x + DIRECTIONS[direction].0;
        let next_y = y + DIRECTIONS[direction].1;
        if next_x >= 0
            && next_y >= 0
            && next_x < max_x
            && next_y < max_y
            && map[next_x as usize][next_y as usize] == b'#'
        {
            // turn
            direction = (direction + 1) % 4;
        }
        // walk
        x += DIRECTIONS[direction].0;
        y += DIRECTIONS[direction].1;
    }
    map[start_x as usize][start_y as usize] = b'^'; // we should not place the obstacle here

    let mut count: usize = 0;
    // try all possible obstacles but only those with an X
    for i in 0..max_x {
        'inner: for j in 0..max_y {
            // Create a deep copy
            let mut map2: Vec<Vec<u8>> = original_map.to_vec();
            if map[i as usize][j as usize] == b'X' {
                map2[i as usize][j as usize] = b'#';
                x = start_x;
                y = start_y;
                direction = 0;
                // now walk the new, modified map
                while x >= 0 && y >= 0 && x < max_x && y < max_y {
                    // we use a bitmap placed in the map for rembembering the directions
                    // we have used at this position
                    if map2[x as usize][y as usize] > 0b1111u8 {
                        // we have not yet stored a direction, just store it
                        map2[x as usize][y as usize] = 1 << direction;
                    } else {
                        // have we been here in same direction
                        if map2[x as usize][y as usize] & (0b1 << direction) > 0 {
                            count += 1;
                            continue 'inner;
                        } else {
                            // add direction
                            map2[x as usize][y as usize] |= 0b1 << direction;
                        }
                    }
                    // check if we need to change direction
                    let next_x = x + DIRECTIONS[direction].0;
                    let next_y = y + DIRECTIONS[direction].1;
                    if next_x >= 0
                        && next_y >= 0
                        && next_x < max_x
                        && next_y < max_y
                        && map2[next_x as usize][next_y as usize] == b'#'
                    {
                        // turn
                        direction = (direction + 1) % 4;
                    } else {
                        // walk
                        x += DIRECTIONS[direction].0;
                        y += DIRECTIONS[direction].1;
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
        assert_eq!(part1(), 5239);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 1753);
    }
}
