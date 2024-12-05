const INPUT: &str = include_str!("../../inputs/input05.txt");

pub fn part1() -> u64 {
    let orderings: Vec<(u8, u8)> = INPUT
        .lines()
        .filter(|line| line.len() == 5 && line.as_bytes().get(2) == Some(&b'|'))
        .map(|line| {
            let mut pair = line.split('|');
            (
                pair.next().unwrap().parse::<u8>().unwrap(),
                pair.next().unwrap().parse::<u8>().unwrap(),
            )
        })
        .collect();
    let updates: Vec<Vec<u8>> = INPUT
        .lines()
        .filter(|line| !line.is_empty() && line.as_bytes().get(2) != Some(&b'|'))
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect();
    let mut middlesum: u64 = 0;
    'nextupdate: for update in updates {
        for (page1, page2) in &orderings {
            // page1 in update
            if let Some(pos1) = update.iter().position(|&x| x == *page1) {
                // page2 in update
                if let Some(pos2) = update.iter().position(|&x| x == *page2) {
                    if pos1 < pos2 {
                        continue;
                    } else {
                        continue 'nextupdate;
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }
        middlesum += update[update.len() / 2] as u64;
    }
    middlesum
}

pub fn part2() -> u64 {
    let orderings: Vec<(u8, u8)> = INPUT
        .lines()
        .filter(|line| line.len() == 5 && line.as_bytes().get(2) == Some(&b'|'))
        .map(|line| {
            let mut pair = line.split('|');
            (
                pair.next().unwrap().parse::<u8>().unwrap(),
                pair.next().unwrap().parse::<u8>().unwrap(),
            )
        })
        .collect();
    let updates: Vec<Vec<u8>> = INPUT
        .lines()
        .filter(|line| !line.is_empty() && line.as_bytes().get(2) != Some(&b'|'))
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect();
    let mut middlesum: u64 = 0;
    // find those with wrong order
    let mut invalid_updates: Vec<Vec<u8>> = Vec::new();
    'nextupdate: for update in updates {
        for (page1, page2) in &orderings {
            // page1 in update
            if let Some(pos1) = update.iter().position(|&x| x == *page1) {
                // page2 in update
                if let Some(pos2) = update.iter().position(|&x| x == *page2) {
                    if pos1 < pos2 {
                        continue;
                    } else {
                        invalid_updates.push(update);
                        continue 'nextupdate;
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }
    }
    for update in invalid_updates.iter_mut() {
        //println!("unordered update: {:?}", update);
        let mut updated = true;
        while updated {
            updated = false;
            for (page1, page2) in &orderings {
                // page1 in update
                if let Some(pos1) = update.iter().position(|&x| x == *page1) {
                    // page2 in update
                    if let Some(pos2) = update.iter().position(|&x| x == *page2) {
                        if pos1 < pos2 {
                            // already correct
                            continue;
                        } else {
                            //println!("violated rule: ({},{})", page1, page2);
                            // reorder move second element directly after first
                            update.remove(pos2);
                            update.insert(pos1, *page2);
                            updated = true;
                        }
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            }
        }
        //println!("reordered update: {:?}", update);
        middlesum += update[update.len() / 2] as u64;
    }
    middlesum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 5374);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 4260);
    }
}
