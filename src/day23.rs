use std::collections::{HashMap, HashSet, VecDeque};
use regex::Regex;

#[aoc_generator(day23)]
pub fn load_input(input: &str) -> VecDeque<usize> {
    let mut output = VecDeque::new();
    for c in input.chars() {
        output.push_back(c.to_string().parse::<usize>().unwrap());
    }
    output
}

pub fn cup_move(cups: &mut VecDeque<usize>) {
    // Pick up 3 cups clockwise of current, remove
    // We shall rotate cups such that current_idx is always at start of
    // VecDeque, making this step trivial.
    let val1 = cups.remove(1).unwrap();
    let val2 = cups.remove(1).unwrap();
    let val3 = cups.remove(1).unwrap();

    // Destination cup selected, cup with label == current cup label - 1,
    // decrement until valid cup found, wrap to highest when below lowest
    let mut dst_value = cups.front().unwrap() - 1;
    loop {
        if val1 == dst_value || val2 == dst_value || val3 == dst_value {
            dst_value -= 1;
        } else if dst_value < 1 {
            dst_value = cups.len() + 3;
        } else {
            break;
        }
    }

    // Place 3 cups clockwise of destination cup in same order as picked up
    let mut dst_idx = 0;
    for v in cups.iter() {
        if *v == dst_value {
            break;
        }
        dst_idx += 1;
    }
    cups.insert(dst_idx + 1, val3);
    cups.insert(dst_idx + 1, val2);
    cups.insert(dst_idx + 1, val1);

    // new current cup selected, immediately clockwise of current cup
    cups.rotate_left(1);
}

#[aoc(day23, part1)]
pub fn part1(input: &VecDeque<usize>) -> String {

    let mut cups = input.clone();

    for _ in 0..100 {
        cup_move(&mut cups);
    }

    // Starting after cup labeled 1, collect labels clockwise. No 1 should be
    // present in output
    loop {
        if *cups.front().unwrap() == 1 {
            break;
        } else {
            cups.rotate_left(1);
        }
    }
    cups.iter().skip(1).map(|&x| x.to_string()).fold("".to_string(), |mut acc, x| {
        acc.push_str(&x);
        return acc;
    })
}

#[aoc(day23, part2)]
pub fn part2(input: &VecDeque<usize>) -> usize {

    let mut cups = input.clone();
    for i in 11..1_000_001 {
        cups.push_back(i);
    }

    for i in 0..10_000_000 {
        if i % 1000 == 0 {
            println!("i: {}", i);
        }
        cup_move(&mut cups);
    }

    // Starting after cup labeled 1, collect labels clockwise. No 1 should be
    // present in output
    loop {
        if *cups.front().unwrap() == 1 {
            break;
        } else {
            cups.rotate_left(1);
        }
    }

    println!("Solved");
    let v1 = cups.get(1).unwrap();
    let v2 = cups.get(2).unwrap();
    println!("{}", v1);
    println!("{}", v2);

    v1 * v2
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/23a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/23a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 0);
    }
}
