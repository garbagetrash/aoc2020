use std::collections::{HashMap, HashSet};
use regex::Regex;

#[derive(Debug)]
pub struct Tile {
    id: usize,
    pixels: [[u8; 10]; 10],
    edges: [u8; 4],
}

#[aoc_generator(day20)]
pub fn load_input(input: &str) -> Vec<Tile> {
    let mut output = vec![];
    let tiles: Vec<_> = input.split("\n\n").collect();
    for tile in tiles {

        let id = input.lines().take(1).split(" :").nth(1).unwrap().parse().unwrap();

        let mut pixels = [[0; 10]; 10];
        let mut sides = vec![];
        for (j, line) in input.lines().skip(1).enumerate() {
            for (i, c) in line.chars().enumerate() {
                pixels[i][j] = c;
            }
        }

        // Top row
        let mut val = 0;
        for i in 0..10 {
            let value = match pixels[0][i] {
                '#' => val += 2.pow(i),
                _ => 0,
            }
            val += value;
        }
        edges.push(val);

        // Right column
        let mut val = 0;
        for i in 0..10 {
            let value = match pixels[i][9] {
                '#' => val += 2.pow(i),
                _ => 0,
            }
            val += value;
        }
        edges.push(val);

        // Bottom row
        let mut val = 0;
        for i in 0..10 {
            let value = match pixels[9][i] {
                '#' => val += 2.pow(i),
                _ => 0,
            }
            val += value;
        }
        edges.push(val);

        // Left column
        let mut val = 0;
        for i in 0..10 {
            let value = match pixels[i][0] {
                '#' => val += 2.pow(i),
                _ => 0,
            }
            val += value;
        }
        edges.push(val);

        output.push( Tile { id: id, edges: edges, pixels: pixels } );
    }
    output
}

#[aoc(day20, part1)]
pub fn part1(input: &[u64]) -> u64 {

    println!("{:?}", input);
    0
}

#[aoc(day20, part2)]
pub fn part2(input: &[u64]) -> u64 {

    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/20a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/20a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 0);
    }
}
