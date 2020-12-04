#![allow(clippy::many_single_char_names)]

pub enum Tile {
    Open,
    Tree,
}

#[aoc_generator(day3)]
pub fn load_input(input: &str) -> Vec<Vec<Tile>> {
    let mut output = vec![];
    for line in input.lines() {
        let mut outline = vec![];
        for c in line.chars() {
            if c == '.' {
                outline.push(Tile::Open);
            } else if c == '#' {
                outline.push(Tile::Tree);
            }
        }
        output.push(outline);
    }

    output
}

pub fn doit(input: &[Vec<Tile>], right: usize, down: usize) -> usize {
    let height = input.len();
    let width = input[0].len();

    let mut treecntr = 0;
    let mut right_cnt = 0;
    let mut i = 0;
    while i < height {
        if let Tile::Tree = input[i][right_cnt % width] {
            treecntr += 1
        };
        right_cnt += right;
        i += down;
    }
    treecntr
}

#[aoc(day3, part1)]
pub fn part1(input: &[Vec<Tile>]) -> usize {
    doit(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn part2(input: &[Vec<Tile>]) -> usize {
    let a = doit(input, 1, 1);
    let b = doit(input, 3, 1);
    let c = doit(input, 5, 1);
    let d = doit(input, 7, 1);
    let e = doit(input, 1, 2);
    a * b * c * d * e
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/03a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(doit(&input, 3, 1), 7);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/03a.txt").unwrap();
        let input = load_input(&input);
        let a = doit(&input, 1, 1);
        let b = doit(&input, 3, 1);
        let c = doit(&input, 5, 1);
        let d = doit(&input, 7, 1);
        let e = doit(&input, 1, 2);
        assert_eq!(a * b * c * d * e, 336);
    }
}
