use std::collections::HashSet;

#[aoc_generator(day17)]
pub fn load_input(input: &str) -> HashSet<(i64, i64, i64)> {
    let mut output: HashSet<(i64, i64, i64)> = HashSet::new();
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if let '#' = c {
                output.insert((x as i64, y as i64, 0));
            }
        }
    }
    output
}

pub fn get_neighbors(pos: &(i64, i64, i64)) -> Vec<(i64, i64, i64)> {
    let mut dirs = vec![];
    for x in -1..2 {
        for y in -1..2 {
            for z in -1..2 {
                dirs.push((x, y, z));
            }
        }
    }

    let dirs: Vec<(i64, i64, i64)> = dirs.iter().filter(|&&x| x != (0, 0, 0)).copied().collect();

    let mut output = vec![];
    for dir in dirs {
        output.push((pos.0 + dir.0, pos.1 + dir.1, pos.2 + dir.2));
    }
    output
}

pub fn count_active_neighbors(key: &(i64, i64, i64), map: &HashSet<(i64, i64, i64)>) -> i64 {
    let mut active_cntr = 0;
    for loc in get_neighbors(key) {
        if map.get(&loc).is_some() {
            active_cntr += 1;
        }
    }
    active_cntr
}

pub fn step(set: &HashSet<(i64, i64, i64)>) -> HashSet<(i64, i64, i64)> {
    let mut candidates: HashSet<(i64, i64, i64)> = HashSet::new();
    for k in set.iter() {
        candidates.insert(*k);
        for loc in get_neighbors(k) {
            candidates.insert(loc);
        }
    }

    let mut new_set: HashSet<(i64, i64, i64)> = HashSet::new();
    for loc in candidates {
        // For each candidate, count its active neighbors
        let active_cntr = count_active_neighbors(&loc, set);

        // Add entry to new set
        if set.get(&loc).is_some() {
            // If currently Active...
            if active_cntr == 2 || active_cntr == 3 {
                new_set.insert(loc);
            }
        } else {
            // If currently Inactive...
            if active_cntr == 3 {
                new_set.insert(loc);
            }
        }
    }
    new_set
}

#[aoc(day17, part1)]
pub fn part1(input: &HashSet<(i64, i64, i64)>) -> usize {
    let mut last_set: HashSet<(i64, i64, i64)> = input.clone();
    for _ in 0..6 {
        let new_set = step(&last_set);
        last_set = new_set;
    }

    last_set.len()
}

pub fn get_neighbors2(pos: &(i64, i64, i64, i64)) -> Vec<(i64, i64, i64, i64)> {
    let mut dirs = vec![];
    for x in -1..2 {
        for y in -1..2 {
            for z in -1..2 {
                for w in -1..2 {
                    dirs.push((x, y, z, w));
                }
            }
        }
    }

    let dirs: Vec<(i64, i64, i64, i64)> = dirs
        .iter()
        .filter(|&&x| x != (0, 0, 0, 0))
        .copied()
        .collect();

    let mut output = vec![];
    for dir in dirs {
        output.push((pos.0 + dir.0, pos.1 + dir.1, pos.2 + dir.2, pos.3 + dir.3));
    }
    output
}

pub fn count_active_neighbors2(
    key: &(i64, i64, i64, i64),
    set: &HashSet<(i64, i64, i64, i64)>,
) -> i64 {
    let mut active_cntr = 0;
    for loc in get_neighbors2(key) {
        if set.get(&loc).is_some() {
            active_cntr += 1;
        }
    }
    active_cntr
}

pub fn step2(set: &HashSet<(i64, i64, i64, i64)>) -> HashSet<(i64, i64, i64, i64)> {
    let mut candidates: HashSet<(i64, i64, i64, i64)> = HashSet::new();
    for k in set.iter() {
        candidates.insert(*k);
        for loc in get_neighbors2(k) {
            candidates.insert(loc);
        }
    }

    let mut new_set: HashSet<(i64, i64, i64, i64)> = HashSet::new();
    for loc in candidates {
        // For each candidate, count its active neighbors
        let active_cntr = count_active_neighbors2(&loc, set);

        // Add entry to new set
        if set.get(&loc).is_some() {
            // If currently Active...
            if active_cntr == 2 || active_cntr == 3 {
                new_set.insert(loc);
            }
        } else {
            // If currently Inactive...
            if active_cntr == 3 {
                new_set.insert(loc);
            }
        }
    }
    new_set
}

#[aoc(day17, part2)]
pub fn part2(input: &HashSet<(i64, i64, i64)>) -> usize {
    let mut last_set: HashSet<(i64, i64, i64, i64)> = HashSet::new();
    for loc in input {
        last_set.insert((loc.0, loc.1, loc.2, 0));
    }

    for _ in 0..6 {
        let new_set = step2(&last_set);
        last_set = new_set;
    }

    last_set.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/17a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 112);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/17a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 848);
    }
}
