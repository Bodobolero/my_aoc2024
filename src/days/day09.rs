#[allow(dead_code)] // not yet used in template
const INPUT: &str = include_str!("../../inputs/input09.txt");

pub fn part1() -> usize {
    let mut disk: Vec<Option<u64>> = Vec::with_capacity(INPUT.len() * 10);
    let mut disk_pos: usize = 0;
    let mut id: u64 = 0;
    // uncompress
    for (pos, char) in INPUT.as_bytes().into_iter().enumerate() {
        if pos % 2 == 0 {
            for _i in 0..(char - b'0') as usize {
                disk.push(Some(id));
            }
            id += 1;
        } else {
            for _i in 0..(char - b'0') as usize {
                disk.push(None);
            }
        }
        disk_pos += (char - b'0') as usize;
    }
    // move
    disk_pos -= 1;
    let mut front: usize = 0;
    while disk_pos > front {
        if disk[front].is_none() {
            while disk[disk_pos].is_none() {
                disk_pos -= 1;
            }
            disk[front] = disk[disk_pos];
            disk[disk_pos] = None;
            disk_pos -= 1;
        }
        front += 1;
    }
    // println!("{:?}", disk);
    disk.iter()
        .enumerate()
        .map(|(index, num)| match num {
            Some(val) => index * *val as usize,
            None => 0,
        })
        .sum()
}

pub fn part2() -> usize {
    let mut files: Vec<(Option<usize>, u8)> = Vec::with_capacity(INPUT.len());
    // layout
    for (pos, char) in INPUT.as_bytes().into_iter().enumerate() {
        if pos % 2 == 0 {
            files.push((Some(pos / 2), (*char - b'0')));
        } else {
            files.push((None, (*char - b'0')));
        }
    }
    // move
    'outer: for i in (0..files.len()).rev() {
        if files[i].0.is_some() {
            for j in 0..i {
                if files[j].0.is_none() && files[j].1 == files[i].1 {
                    files.swap(i, j);
                    continue 'outer;
                }
                if files[j].0.is_none() && files[j].1 > files[i].1 {
                    let rest = files[j].1 - files[i].1;
                    files[j].1 = files[i].1;
                    files.swap(i, j);
                    if files[j + 1].0.is_none() {
                        files[j + 1].1 += rest;
                    } else {
                        files.insert(j + 1, (None, rest));
                    }

                    continue 'outer;
                }
            }
        }
    }
    // println!("{:?}", files);

    // count
    let mut pos: usize = 0;
    let mut count: usize = 0;
    for i in 0..files.len() {
        if let Some(val) = files[i].0 {
            for _j in 0..files[i].1 {
                count += pos * val;
                pos += 1;
            }
        } else {
            pos += files[i].1 as usize;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 6461289671426);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 6488291456470);
    }
}
