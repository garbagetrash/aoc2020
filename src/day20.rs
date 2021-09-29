use regex::Regex;
use std::cmp;
use std::collections::{HashMap, HashSet};
use num::integer::Roots;

#[derive(Debug)]
pub struct Tile {
    id: u64,
    pixels: [[char; 10]; 10],

    // N, E, S, W
    edges: [u16; 4],
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
        let mut val = 0_u16;
        for i in 0..10 {
            match pixels[i][0] {
                '#' => val += 2_u16.pow((9 - i) as u32),
                _ => (),
            };
        }
        edges[0] = val;

        // Right column
        let mut val = 0_u16;
        for i in 0..10 {
            match pixels[9][i] {
                '#' => val += 2_u16.pow((9 - i) as u32),
                _ => (),
            };
        }
        edges[1] = val;

        // Bottom row
        let mut val = 0_u16;
        for i in 0..10 {
            match pixels[i][9] {
                '#' => val += 2_u16.pow((9 - i) as u32),
                _ => (),
            };
        }
        edges[2] = val;

        // Left column
        let mut val = 0_u16;
        for i in 0..10 {
            match pixels[0][i] {
                '#' => val += 2_u16.pow((9 - i) as u32),
                _ => (),
            };
        }
        edges[3] = val;

        output.push(Tile {
            id: id,
            edges: edges,
            pixels: pixels,
        });
    }
    output
}

pub fn flip_number(num: &u16) -> u16 {
    let nstr = format!("{:010b}", num);
    let mut bitvec: Vec<u16> = nstr.chars().map(|c| c.to_digit(10).unwrap() as u16).collect();
    bitvec.reverse();
    let bitstr = bitvec.iter().map(|b| b.to_string()).fold("".to_string(), |mut acc, b| {
        acc.push_str(&b);
        return acc;
    });
    u16::from_str_radix(&bitstr, 2).unwrap()
}

// Returns IDs of tiles that can attach to reference tile
pub fn find_neighbors(ref_tile: &Tile, tileset: &[Tile]) -> Vec<u64> {
    let mut output = vec![];
    for edge in ref_tile.edges {
        for candidate in tileset {
            for e2 in candidate.edges {
                if e2 == edge {
                    // This candidate can connect to our ref_tile
                    output.push(candidate.id);
                }
            }
        }
    }
    output
}

// Return tile IDs from the tileset that have the passed reference edge.
pub fn get_tiles_with_edge(ref_edge: u16, tileset: &[Tile]) -> HashSet<u64> {
    let ref_other = flip_number(&ref_edge);

    let mut output = HashSet::new();
    for tile in tileset {
        for edge in tile.edges {
            if edge == ref_edge || edge == ref_other {
                output.insert(tile.id);
            }
        }
    }
    output
}

#[aoc(day20, part1)]
pub fn part1(input: &[Tile]) -> u64 {

    let edge_len = input.len().sqrt();

    // Get all the unique edges
    let mut edge_set = HashSet::new();
    for tile in input {
        for edge in &tile.edges {
            let other = flip_number(edge);
            let key = cmp::min(*edge, other);
            edge_set.insert(key);
        }
    }

    // Now get all Tiles that have a given edge
    let mut edge_tile_map = HashMap::new();
    let mut unique_edge_cnt = HashMap::<u64, u64>::new();
    for edge in edge_set.iter() {
        let tiles = get_tiles_with_edge(*edge, input);

        if tiles.len() == 1 {
            if let Some(cnt) = unique_edge_cnt.get_mut(tiles.iter().next().unwrap()) {
                *cnt += 1;
            } else {
                unique_edge_cnt.insert(*tiles.iter().next().unwrap(), 1);
            }
        }
        edge_tile_map.insert(edge, tiles);
    }

    let mut corners: Vec<u64> = vec![];
    for (tile_id, cnt) in unique_edge_cnt.iter() {
        if *cnt > 1 {
            // This tile has more than 1 unique edge, it must be a corner.
            corners.push(*tile_id);
        }
    }

    corners.iter().product()
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
    fn test_flip_number() {
        let num: u16 = 832;
        let other = flip_number(&num);
        assert_eq!(other, 11);

        let num: u16 = 241;
        let other = flip_number(&num);
        assert_eq!(other, 572);

        let num: u16 = 572;
        let other = flip_number(&num);
        assert_eq!(other, 241);
    }

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
