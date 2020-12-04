#[aoc_generator(dayN)]
pub fn load_input(input: &str) -> Vec<u64> {
    let mut output = vec![];
    for line in input.lines() {
        output.push(line.parse().unwrap());
    }
    output
}

#[aoc(dayN, part1)]
pub fn part1(input: &[u64]) -> u64 {

    0
}

#[aoc(dayN, part2)]
pub fn part2(input: &[u64]) -> u64 {

    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/01a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/01a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 0);
    }
}
