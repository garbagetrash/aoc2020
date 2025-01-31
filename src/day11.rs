use std::collections::HashMap;
use std::{thread, time};

use ncurses::*;

#[aoc_generator(day11)]
pub fn load_input(input: &str) -> HashMap<(i32, i32), Tile> {
    let mut output = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '.' => Tile::Floor,
                'L' => Tile::Empty,
                _ => Tile::Empty,
            };
            output.insert((x as i32, y as i32), tile);
        }
    }
    output
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Tile {
    Floor,
    Empty,
    Occupied,
}

#[derive(PartialEq)]
pub enum PartSwitch {
    Part1,
    Part2,
}

pub fn count_occupied(
    pos: (i32, i32),
    ferry: &HashMap<(i32, i32), Tile>,
    part_switch: &PartSwitch,
) -> i32 {
    let mut cnt = 0;
    let dirs = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (-1, -1),
        (1, -1),
        (1, 1),
        (-1, 1),
    ];

    for dir in dirs.iter() {
        let mut tempdir = *dir;
        while let Some(t) = ferry.get(&(pos.0 + tempdir.0, pos.1 + tempdir.1)) {
            match t {
                Tile::Occupied => {
                    cnt += 1;
                    break;
                }
                Tile::Empty => break,
                Tile::Floor => {
                    if *part_switch == PartSwitch::Part1 {
                        break;
                    }
                    tempdir.0 += dir.0;
                    tempdir.1 += dir.1;
                }
            }
        }
    }

    cnt
}

pub fn apply(
    pos: (i32, i32),
    ferry: &HashMap<(i32, i32), Tile>,
    part_switch: &PartSwitch,
) -> Option<Tile> {
    let num_seats = match *part_switch {
        PartSwitch::Part1 => 4,
        PartSwitch::Part2 => 5,
    };

    let mut new_tile = None;
    if let Some(t) = ferry.get(&(pos.0, pos.1)) {
        match t {
            Tile::Empty => {
                if count_occupied(pos, ferry, part_switch) == 0 {
                    new_tile = Some(Tile::Occupied);
                }
            }
            Tile::Occupied => {
                if count_occupied(pos, ferry, part_switch) >= num_seats {
                    new_tile = Some(Tile::Empty);
                }
            }
            _ => (),
        }
    }
    new_tile
}

pub fn compare_ferries(f1: &HashMap<(i32, i32), Tile>, f2: &HashMap<(i32, i32), Tile>) -> bool {
    for k in f1.keys() {
        if f1.get(k) != f2.get(k) {
            return false;
        }
    }
    true
}

pub fn apply_rules(
    ferry: &HashMap<(i32, i32), Tile>,
    part_switch: &PartSwitch,
) -> HashMap<(i32, i32), Tile> {
    let mut new_ferry = ferry.clone();
    for (pos, tile) in ferry.iter() {
        match tile {
            Tile::Empty => {
                if let Some(new_tile) = apply(*pos, &ferry, part_switch) {
                    if let Some(x) = new_ferry.get_mut(pos) {
                        *x = new_tile;
                    }
                }
            }
            Tile::Occupied => {
                if let Some(new_tile) = apply(*pos, &ferry, part_switch) {
                    if let Some(x) = new_ferry.get_mut(pos) {
                        *x = new_tile;
                    }
                }
            }
            _ => (),
        }
    }
    new_ferry
}

pub fn occupied_count(ferry: &HashMap<(i32, i32), Tile>) -> u64 {
    let mut cnt = 0;
    for value in ferry.values() {
        if *value == Tile::Occupied {
            cnt += 1;
        }
    }
    cnt
}

pub fn print_ferry(ferry: &HashMap<(i32, i32), Tile>) {
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    clear();

    for ((x, y), tile) in ferry.iter() {
        match tile {
            Tile::Floor => mvprintw(*y, *x, "."),
            Tile::Empty => mvprintw(*y, *x, "L"),
            Tile::Occupied => mvprintw(*y, *x, "#"),
        };
    }
    refresh();
    thread::sleep(time::Duration::from_millis(100));
    curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
    endwin();
}

pub fn doit(input: &HashMap<(i32, i32), Tile>, part_switch: PartSwitch) -> u64 {
    let mut ferry = input.clone();

    loop {
        let new_ferry = apply_rules(&ferry, &part_switch);
        //print_ferry(&new_ferry);

        if compare_ferries(&ferry, &new_ferry) {
            // Done
            return occupied_count(&ferry);
        }
        ferry = new_ferry;
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &HashMap<(i32, i32), Tile>) -> u64 {
    doit(input, PartSwitch::Part1)
}

#[aoc(day11, part2)]
pub fn part2(input: &HashMap<(i32, i32), Tile>) -> u64 {
    doit(input, PartSwitch::Part2)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/11a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 37);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/11b.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 26);
    }
}
