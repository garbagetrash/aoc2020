#![allow(clippy::many_single_char_names)]
use std::fs::read_to_string;

pub enum Tile {
    Open,
    Tree,
}

pub fn load_input(filename: &str) -> Vec<Vec<Tile>> {
    let buf = read_to_string(filename).unwrap();

    let mut output = vec![];
    for line in buf.lines() {
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
        if let Tile::Tree = input[i][right_cnt % width] { treecntr += 1 };
        right_cnt += right;
        i += down;
    }
    treecntr
}

pub fn part1(input: &[Vec<Tile>]) -> usize {
    doit(input, 3, 1)
}

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

    #[test]
    fn test_part1() {
        let input = load_input("inputs/03a.txt");
        assert_eq!(doit(&input, 3, 1), 7);
    }

    #[test]
    fn test_part2() {
        let input = load_input("inputs/03a.txt");
        let a = doit(&input, 1, 1);
        let b = doit(&input, 3, 1);
        let c = doit(&input, 5, 1);
        let d = doit(&input, 7, 1);
        let e = doit(&input, 1, 2);
        assert_eq!(a * b * c * d * e, 336);
    }
}
