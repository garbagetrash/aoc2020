use regex::Regex;
use std::collections::{HashMap, HashSet};
use num::integer::Roots;

#[derive(Debug)]
pub struct Tile {
    id: usize,
    pixels: [[char; 10]; 10],
    edges: [u8; 4],
}

#[aoc_generator(day20)]
pub fn load_input(input: &str) -> Vec<Tile> {
    let mut output = vec![];
    let tiles: Vec<_> = input.split("\n\n").collect();
    for tile in tiles {

        let id = tile
            .lines()
            .take(1)
            .collect::<Vec<_>>()[0]
            .split_whitespace()
            .nth(1)
            .unwrap()
            .strip_suffix(":")
            .unwrap()
            .parse()
            .unwrap();

        let mut pixels = [['.'; 10]; 10];
        for (j, line) in tile.lines().skip(1).enumerate() {
            for (i, c) in line.chars().enumerate() {
                pixels[i][j] = c;
            }
        }

        let mut edges = [0; 4];
        // Top row
        let mut val = 0_u8;
        for i in 0..10 {
            match pixels[0][i] {
                '#' => val += 2_u8.pow(i as u32),
                _ => (),
            };
        }

        // We arbitrarily decide we'll use the lesser valued flip of the
        // edge just so we have some repeatable rule.
        let other = flip_number(&val);
        if other < val {
            edges[0] = other;
        } else {
            edges[0] = val;
        }

        // Right column
        let mut val = 0_u8;
        for i in 0..10 {
            match pixels[i][9] {
                '#' => val += 2_u8.pow(i as u32),
                _ => (),
            };
        }
        let other = flip_number(&val);
        if other < val {
            edges[1] = other;
        } else {
            edges[1] = val;
        }

        // Bottom row
        let mut val = 0_u8;
        for i in 0..10 {
            match pixels[9][i] {
                '#' => val += 2_u8.pow(i as u32),
                _ => (),
            };
        }
        let other = flip_number(&val);
        if other < val {
            edges[2] = other;
        } else {
            edges[2] = val;
        }

        // Left column
        let mut val = 0_u8;
        for i in 0..10 {
            match pixels[i][0] {
                '#' => val += 2_u8.pow(i as u32),
                _ => (),
            };
        }
        let other = flip_number(&val);
        if other < val {
            edges[3] = other;
        } else {
            edges[3] = val;
        }

        output.push(Tile {
            id: id,
            edges: edges,
            pixels: pixels,
        });
    }
    output
}

pub fn flip_number(num: &u8) -> u8 {
    let nstr = format!("{:08b}", num);
    let mut bitvec: Vec<u8> = nstr.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
    bitvec.reverse();
    let bitstr = bitvec.iter().map(|b| b.to_string()).fold("".to_string(), |mut acc, b| {
        acc.push_str(&b);
        return acc;
    });
    u8::from_str_radix(&bitstr, 2).unwrap()
}

#[aoc(day20, part1)]
pub fn part1(input: &[Tile]) -> u64 {

    println!("num tiles: {}", input.len());
    let edge_len = input.len().sqrt();
    println!("edge length: {}", edge_len);

    let testnum = 15_u8;
    let newnum = flip_number(&testnum);
    println!("newnum: {}", newnum);

    // Border edges don't align with any other tile, meaning we can id corners
    // by looking for pieces with 2 unique border edges... don't have to solve
    // puzzle yet!
    let mut edge_cnt = HashMap::new();
    for tile in input {
        for edge in &tile.edges {
            // We don't have to worry about edge flips since we normalized them
            // in the input parsing section
            if let Some(e) = edge_cnt.get_mut(&edge) {
                *e += 1;
            } else {
                edge_cnt.insert(edge, 1);
            }
        }
    }

    for (k, v) in edge_cnt.iter() {
        if v == &1 {
            println!("Edge only seen once, _must_ be an outside border: {:?}", k);
        } else if v % 2 == 1 {
            println!("Edge seen odd number of times, must be an outside border at least once: {:?}", k);
        }
    }

    let mut mult_vec = vec![];
    for tile in input {
        let mut unique_edge_cntr = 0;
        for edge in &tile.edges {
            if *edge_cnt.get(&edge).unwrap() % 2 == 1 {
                // We have either a side or corner piece...
                unique_edge_cntr += 1;
            }
        }
        if unique_edge_cntr >= 2 {
            // Must be a corner!
            mult_vec.push(tile.id);
        }
    }

    println!("{:?}", edge_cnt);
    println!("{:?}", mult_vec);
    mult_vec.iter().fold(1, |acc, x| acc * *x as u64)
}

#[aoc(day20, part2)]
pub fn part2(input: &[Tile]) -> u64 {
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
