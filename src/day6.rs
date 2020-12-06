use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn load_input(input: &str) -> Vec<(HashMap<char, u32>, u32)> {
    let mut output = vec![];
    for group in input.split("\n\n") {
        let mut gmap = HashMap::new();
        for c in "abcdefghijklmnopqrstuvwxyz".chars() {
            gmap.insert(c, 0);
        }

        let mut npeople = 0;
        for person in group.lines() {
            npeople += 1;
            for c in person.chars() {
                if let Some(x) = gmap.get_mut(&c) {
                    *x += 1;
                }
            }
        }
        output.push((gmap, npeople));
    }
    output
}

#[aoc(day6, part1)]
pub fn part1(input: &[(HashMap<char, u32>, u32)]) -> u32 {
    input
        .iter()
        .map(|(gmap, _)| {
            let mut accum = 0;
            for v in gmap.values() {
                if v > &0 {
                    accum += 1;
                }
            }
            accum
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &[(HashMap<char, u32>, u32)]) -> usize {
    let mut accum = 0;
    for (gmap, npeople) in input {
        for v in gmap.values() {
            if v == npeople {
                accum += 1;
            }
        }
    }
    accum
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/6a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 11);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/6a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 6);
    }
}
