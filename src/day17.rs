use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum State {
    Active,
    Inactive,
}

#[aoc_generator(day17)]
pub fn load_input(input: &str) -> HashMap<(i64, i64, i64), State> {
    let mut output: HashMap<(i64, i64, i64), State> = HashMap::new();
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    output.insert((x as i64, y as i64, 0), State::Active);
                }
                '.' => {
                    output.insert((x as i64, y as i64, 0), State::Inactive);
                }
                _ => (),
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

pub fn count_active_neighbors(key: &(i64, i64, i64), map: &HashMap<(i64, i64, i64), State>) -> i64 {
    let mut active_cntr = 0;
    for loc in get_neighbors(key) {
        if let Some(v) = map.get(&loc) {
            if let State::Active = v {
                active_cntr += 1;
            }
        }
    }
    active_cntr
}

pub fn step(map: &HashMap<(i64, i64, i64), State>) -> HashMap<(i64, i64, i64), State> {
    let mut candidates: HashSet<(i64, i64, i64)> = HashSet::new();
    for (k, _v) in map.iter().filter(|(_k, &v)| v == State::Active) {
        candidates.insert(*k);
        for loc in get_neighbors(k) {
            candidates.insert(loc);
        }
    }

    let mut new_map: HashMap<(i64, i64, i64), State> = HashMap::new();
    for loc in candidates {
        // For each candidate, count its active neighbors
        let active_cntr = count_active_neighbors(&loc, map);

        // Add entry to new map
        if let Some(v) = map.get(&loc) {
            match v {
                State::Active => {
                    if active_cntr == 2 || active_cntr == 3 {
                        new_map.insert(loc, State::Active);
                    } else {
                        new_map.insert(loc, State::Inactive);
                    }
                }
                State::Inactive => {
                    if active_cntr == 3 {
                        new_map.insert(loc, State::Active);
                    } else {
                        new_map.insert(loc, State::Inactive);
                    }
                }
            }
        } else {
            // Not yet in map then it's Inactive, so apply appropriate logic
            if active_cntr == 3 {
                new_map.insert(loc, State::Active);
            } else {
                new_map.insert(loc, State::Inactive);
            }
        }
    }
    new_map
}

#[aoc(day17, part1)]
pub fn part1(input: &HashMap<(i64, i64, i64), State>) -> u64 {
    let mut last_map: HashMap<(i64, i64, i64), State> = input.clone();
    for _ in 0..6 {
        let new_map = step(&last_map);
        last_map = new_map;
    }

    let mut cntr = 0;
    for state in last_map.values() {
        if *state == State::Active {
            cntr += 1;
        }
    }

    cntr
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
    map: &HashMap<(i64, i64, i64, i64), State>,
) -> i64 {
    let mut active_cntr = 0;
    for loc in get_neighbors2(key) {
        if let Some(v) = map.get(&loc) {
            if let State::Active = v {
                active_cntr += 1;
            }
        }
    }
    active_cntr
}

pub fn step2(map: &HashMap<(i64, i64, i64, i64), State>) -> HashMap<(i64, i64, i64, i64), State> {
    let mut candidates: HashSet<(i64, i64, i64, i64)> = HashSet::new();
    for (k, _v) in map.iter().filter(|(_k, &v)| v == State::Active) {
        candidates.insert(*k);
        for loc in get_neighbors2(k) {
            candidates.insert(loc);
        }
    }

    let mut new_map: HashMap<(i64, i64, i64, i64), State> = HashMap::new();
    for loc in candidates {
        // For each candidate, count its active neighbors
        let active_cntr = count_active_neighbors2(&loc, map);

        // Add entry to new map
        if let Some(v) = map.get(&loc) {
            match v {
                State::Active => {
                    if active_cntr == 2 || active_cntr == 3 {
                        new_map.insert(loc, State::Active);
                    } else {
                        new_map.insert(loc, State::Inactive);
                    }
                }
                State::Inactive => {
                    if active_cntr == 3 {
                        new_map.insert(loc, State::Active);
                    } else {
                        new_map.insert(loc, State::Inactive);
                    }
                }
            }
        } else {
            // Not yet in map then it's Inactive, so apply appropriate logic
            if active_cntr == 3 {
                new_map.insert(loc, State::Active);
            } else {
                new_map.insert(loc, State::Inactive);
            }
        }
    }
    new_map
}

#[aoc(day17, part2)]
pub fn part2(input: &HashMap<(i64, i64, i64), State>) -> u64 {
    let mut last_map: HashMap<(i64, i64, i64, i64), State> = HashMap::new();
    for (k, v) in input {
        last_map.insert((k.0, k.1, k.2, 0), *v);
    }

    for _ in 0..6 {
        let new_map = step2(&last_map);
        last_map = new_map;
    }

    let mut cntr = 0;
    for state in last_map.values() {
        if *state == State::Active {
            cntr += 1;
        }
    }

    cntr
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
