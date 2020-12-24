use std::collections::{HashMap, HashSet};

// Idea:
// Lets make a Vec<usize>.  Vecs have constant access time.  But let's play
// with the idea some.
// Instead of actually moving elements around, let's say the position in the
// Vec actually represents the "Value" of the cup.  Lets say the value stored
// at that position is the position of the cup with the value.  We're kinda
// doing this reverso land.  The idea being we just update position values in
// the Vec but don't _actually_ move any elements around, so everything should
// stay constant access and write.
#[aoc_generator(day23)]
pub fn load_input(input: &str) -> Vec<usize> {
    let mut output = Vec::new();
    for c in input.chars() {
        output.push(c.to_string().parse::<usize>().unwrap());
    }
    output
}

pub fn cup_move(value_vec: &mut Vec<usize>, current_idx: &mut usize) {
    // Pick up 3 cups clockwise of current, remove
    // We shall rotate cups such that current_idx is always at start of
    // Vec, making this step trivial.
    let ncups = value_vec.len();
    let idx0 = (*current_idx + 1) % ncups;
    let idx1 = (*current_idx + 2) % ncups;
    let idx2 = (*current_idx + 3) % ncups;
    let v0 = value_vec[idx0];
    let v1 = value_vec[idx1];
    let v2 = value_vec[idx2];

    println!("\ncups: {:?}", value_vec);
    println!("curr_idx: {}", current_idx);
    println!("pick up: {}, {}, {}", v0, v1, v2);

    // Destination cup selected, cup with label == current cup label - 1,
    // decrement until valid cup found, wrap to highest when below lowest
    let curr_value = value_vec[*current_idx];
    let mut dst_value = curr_value - 1;
    loop {
        if v0 == dst_value || v1 == dst_value || v2 == dst_value {
            dst_value -= 1;
        } else if dst_value < 1 {
            dst_value = ncups;
        } else {
            break;
        }
    }

    println!("destination: {}", dst_value);

    // Place 3 cups clockwise of destination cup in same order as picked up
    let dst_idx = value_vec.iter().position(|&x| x == dst_value).unwrap();
    println!("destination idx: {}", dst_idx);
    if *current_idx < dst_idx {

        // Moving cups to the left
        for i in *current_idx..(dst_idx - 3) {
            let v = value_vec[(i + 4) % ncups];
            value_vec[(i + 1) % ncups] = v;
        }

        // Move 3 cups to new position
        value_vec[(dst_idx - 2) % ncups] = v0;
        value_vec[(dst_idx - 1) % ncups] = v1;
        value_vec[dst_idx] = v2;

    } else {

        // Moving cups to the left
        for i in (*current_idx + 1)..(ncups - 3) {
            let v = value_vec[(i + 3) % ncups];
            value_vec[i] = v;
        }
        let mut end = dst_idx + 3;
        if *current_idx < end {
            end = *current_idx;
        }
        for i in 0..end {
            let v = value_vec[i];
            value_vec[((i as i32 - 3).rem_euclid(ncups as i32)) as usize] = v;
        }

        // Move 3 cups to new position
        value_vec[(dst_idx as i32 - 2).rem_euclid(ncups as i32) as usize] = v0;
        value_vec[(dst_idx as i32 - 1).rem_euclid(ncups as i32) as usize] = v1;
        value_vec[dst_idx] = v2;
    }

    // new current cup selected, immediately clockwise of current cup
    *current_idx += 1;
    *current_idx %= ncups;
}

#[aoc(day23, part1)]
pub fn part1(input: &Vec<usize>) -> String {

    // This Vec holds the cup value, indexes represent positions (this is how
    // our brain wants to think of this problem.)
    let mut value_vec = input.clone();

    println!("{:?}", value_vec);

    let mut current_idx = 0;

    for _ in 0..100 {
        cup_move(&mut value_vec, &mut current_idx);
    }

    println!("{:?}", value_vec);

    // Starting after cup labeled 1, collect labels clockwise. No 1 should be
    // present in output
    let pos = value_vec.iter().position(|&x| x == 1).unwrap();
    value_vec.iter().cycle().skip(pos + 1).take(8).map(|&x| x.to_string()).fold("".to_string(), |mut acc, x| {
        acc.push_str(&x);
        return acc;
    })
}

#[aoc(day23, part2)]
pub fn part2(input: &Vec<usize>) -> usize {

    panic!();

    // This Vec holds the cup value, indexes represent positions (this is how
    // our brain wants to think of this problem.)
    let mut value_vec = input.clone();

    let mut value_vec = input.clone();
    for i in 11..1_000_001 {
        value_vec.push(i);
    }

    let mut current_idx = 0;

    for i in 0..10_000_000 {
        if i % 1000 == 0 {
            println!("i: {}", i);
        }
        cup_move(&mut value_vec, &mut current_idx);
    }

    println!("Solved");

    // Starting after cup labeled 1, collect labels clockwise. No 1 should be
    // present in output
    let pos = value_vec.iter().position(|&x| x == 1).unwrap();

    let v1 = value_vec[(pos + 1) % value_vec.len()];
    let v2 = value_vec[(pos + 2) % value_vec.len()];
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
        let input = load_input("389125467");
        assert_eq!(part1(&input), "67384529".to_string());
    }

    #[test]
    fn test_part2() {
        let input = load_input("389125467");
        assert_eq!(part2(&input), 149245887792);
    }
}
