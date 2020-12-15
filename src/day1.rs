#[aoc_generator(day1)]
pub fn load_input(input: &str) -> Vec<i64> {
    let mut output = vec![];
    for line in input.lines() {
        output.push(line.parse().unwrap());
    }
    output
}

#[aoc(day1, part1)]
pub fn part1(input: &[i64]) -> i64 {
    for v1 in input {
        let other = 2020 - v1;
        if let Some(ans) = input.iter().find(|&&x| x == other) {
            return ans * v1;
        }
    }

    0
}

#[aoc(day1, part2)]
pub fn part2(input: &[i64]) -> i64 {
    for v1 in input {
        for v2 in input {
            let last = 2020 - v1 - v2;
            if let Some(v3) = input.iter().find(|&&x| x == last) {
                return v1 * v2 * v3;
            }
        }
    }

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
        assert_eq!(part1(&input), 514579);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/01a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 241861950);
    }
}
