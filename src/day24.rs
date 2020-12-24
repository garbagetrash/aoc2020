use std::collections::HashSet;

// Coordinate system references:
// https://www.redblobgames.com/grids/hexagons/#coordinates
//
// In particular the Axial Coordinates he describes

#[aoc_generator(day24)]
pub fn load_input(input: &str) -> HashSet<(i64, i64)> {
    let mut output = HashSet::new();
    for line in input.lines() {
        let mut q = 0;
        let mut r = 0;
        let mut line_iter = line.chars();
        while let Some(c) = line_iter.next() {
            match c {
                'e' => q += 1,
                'w' => q -= 1,
                's' => {
                    let l2 = line_iter.next().unwrap();
                    match l2 {
                        'e' => r += 1,
                        'w' => {
                            q -= 1;
                            r += 1;
                        }
                        _ => (),
                    }
                }
                'n' => {
                    let l2 = line_iter.next().unwrap();
                    match l2 {
                        'e' => {
                            q += 1;
                            r -= 1;
                        }
                        'w' => r -= 1,
                        _ => (),
                    }
                }
                _ => (),
            }
        }

        if output.contains(&(q, r)) {
            output.remove(&(q, r));
        } else {
            output.insert((q, r));
        }
    }
    output
}

#[aoc(day24, part1)]
pub fn part1(input: &HashSet<(i64, i64)>) -> usize {
    input.len()
}

pub fn neighbors(pos: &(i64, i64)) -> Vec<(i64, i64)> {
    let dirs = vec![(1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1), (1, -1)];

    let mut output = vec![];
    for dir in dirs {
        output.push((dir.0 + pos.0, dir.1 + pos.1));
    }

    output
}

pub fn count_black_neighbors(pos: &(i64, i64), floor: &HashSet<(i64, i64)>) -> usize {
    let neighbors = neighbors(pos);

    let mut cntr = 0;
    for n in neighbors {
        if floor.contains(&n) {
            cntr += 1;
        }
    }

    cntr
}

pub fn day(floor: &HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut consider_tiles: HashSet<(i64, i64)> = HashSet::new();
    for k in floor.iter() {
        // Any black tiles should be considered
        consider_tiles.insert(*k);

        // Any tiles neighboring black tiles should be considered
        let nset = neighbors(k);
        for n in nset {
            consider_tiles.insert(n);
        }
    }

    // Now we have a set of tiles to consider
    let mut new_floor: HashSet<(i64, i64)> = floor.clone();
    for pos in consider_tiles.iter() {
        let bn = count_black_neighbors(&pos, &floor);
        if floor.contains(&pos) {
            if bn == 0 || bn > 2 {
                // Flip from black to white via removal
                new_floor.remove(&pos);
            }
        } else {
            // Not already in floor map, but still needs considered.  Must be
            // white.
            if bn == 2 {
                // This will turn black now, needs added to set
                new_floor.insert(*pos);
            }
        }
    }

    new_floor
}

#[aoc(day24, part2)]
pub fn part2(input: &HashSet<(i64, i64)>) -> usize {
    let mut floor = input.clone();
    for _ in 0..100 {
        let new_floor = day(&floor);
        floor = new_floor;
    }
    floor.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/24a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 10);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/24a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 2208);
    }
}
