#[derive(Debug, Copy, Clone)]
pub enum Dir {
    North,
    East,
    South,
    West,
    Left,
    Right,
    Front,
}

#[aoc_generator(day12)]
pub fn load_input(input: &str) -> Vec<(Dir, i32)> {
    let mut output = vec![];
    for line in input.lines() {
        let mut dir = Dir::East;
        match line.chars().next().unwrap() {
            'N' => dir = Dir::North,
            'S' => dir = Dir::South,
            'E' => dir = Dir::East,
            'W' => dir = Dir::West,
            'L' => dir = Dir::Left,
            'R' => dir = Dir::Right,
            'F' => dir = Dir::Front,
            _ => (),
        }
        let nsteps = line[1..].parse::<i32>().unwrap();
        output.push((dir, nsteps));
    }
    output
}

pub fn turn(currdir: Dir, turn: Dir, angle: i32) -> Dir {
    let mut newdir = currdir;
    match turn {
        Dir::Left => match angle {
            90 => match currdir {
                Dir::North => newdir = Dir::West,
                Dir::East => newdir = Dir::North,
                Dir::South => newdir = Dir::East,
                Dir::West => newdir = Dir::South,
                _ => (),
            },
            180 => match currdir {
                Dir::North => newdir = Dir::South,
                Dir::East => newdir = Dir::West,
                Dir::South => newdir = Dir::North,
                Dir::West => newdir = Dir::East,
                _ => (),
            },
            270 => match currdir {
                Dir::North => newdir = Dir::East,
                Dir::East => newdir = Dir::South,
                Dir::South => newdir = Dir::West,
                Dir::West => newdir = Dir::North,
                _ => (),
            },
            _ => (),
        },
        Dir::Right => match angle {
            90 => match currdir {
                Dir::North => newdir = Dir::East,
                Dir::East => newdir = Dir::South,
                Dir::South => newdir = Dir::West,
                Dir::West => newdir = Dir::North,
                _ => (),
            },
            180 => match currdir {
                Dir::North => newdir = Dir::South,
                Dir::East => newdir = Dir::West,
                Dir::South => newdir = Dir::North,
                Dir::West => newdir = Dir::East,
                _ => (),
            },
            270 => match currdir {
                Dir::North => newdir = Dir::West,
                Dir::East => newdir = Dir::North,
                Dir::South => newdir = Dir::East,
                Dir::West => newdir = Dir::South,
                _ => (),
            },
            _ => (),
        },
        _ => (),
    }

    newdir
}

pub fn turn2(waypos: (i32, i32), turn: Dir, angle: i32) -> (i32, i32) {
    let mut newwaypos = waypos;
    match turn {
        Dir::Left => match angle {
            90 => newwaypos = (-waypos.1, waypos.0),
            180 => newwaypos = (-waypos.0, -waypos.1),
            270 => newwaypos = (waypos.1, -waypos.0),
            _ => (),
        },
        Dir::Right => match angle {
            90 => newwaypos = (waypos.1, -waypos.0),
            180 => newwaypos = (-waypos.0, -waypos.1),
            270 => newwaypos = (-waypos.1, waypos.0),
            _ => (),
        },
        _ => (),
    }

    newwaypos
}

#[aoc(day12, part1)]
pub fn part1(input: &[(Dir, i32)]) -> i32 {
    let mut pos = (0, 0);
    let mut currdir = Dir::East;
    for (d, s) in input {
        match d {
            Dir::North => {
                pos.1 += s;
            }
            Dir::South => {
                pos.1 -= s;
            }
            Dir::East => {
                pos.0 += s;
            }
            Dir::West => {
                pos.0 -= s;
            }
            Dir::Left => {
                currdir = turn(currdir, Dir::Left, *s);
            }
            Dir::Right => {
                currdir = turn(currdir, Dir::Right, *s);
            }
            Dir::Front => match currdir {
                Dir::North => pos.1 += s,
                Dir::South => pos.1 -= s,
                Dir::East => pos.0 += s,
                Dir::West => pos.0 -= s,
                _ => (),
            },
        }
    }

    pos.0.abs() + pos.1.abs()
}

#[aoc(day12, part2)]
pub fn part2(input: &[(Dir, i32)]) -> i32 {
    let mut shippos = (0, 0);
    let mut waypos = (10, 1);
    for (d, s) in input {
        match d {
            Dir::North => {
                waypos.1 += s;
            }
            Dir::South => {
                waypos.1 -= s;
            }
            Dir::East => {
                waypos.0 += s;
            }
            Dir::West => {
                waypos.0 -= s;
            }
            Dir::Left => {
                waypos = turn2(waypos, Dir::Left, *s);
            }
            Dir::Right => {
                waypos = turn2(waypos, Dir::Right, *s);
            }
            Dir::Front => {
                for _ in 0..*s {
                    shippos.0 += waypos.0;
                    shippos.1 += waypos.1;
                }
            }
        }
    }

    shippos.0.abs() + shippos.1.abs()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/12a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 25);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/12a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 286);
    }
}
