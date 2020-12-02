use std::fs::read_to_string;

pub fn load_input() -> Vec<u64> {
    let buf = read_to_string("inputs/day01.txt").unwrap();

    let mut output = vec![];
    for line in buf.lines() {
        output.push(line.parse().unwrap());
    }
    output
}

pub fn part1(input: &[u64]) -> u64 {
    for v1 in input {
        for v2 in input {
            if v1 + v2 == 2020 && v1 != v2 {
                return v1 * v2;
            }
        }
    }

    0
}

pub fn part2(input: &[u64]) -> u64 {
    for v1 in input {
        for v2 in input {
            for v3 in input {
                if v1 + v2 + v3 == 2020 && v1 != v2 && v1 != v3 && v2 != v3 {
                    return v1 * v2 * v3;
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part1(&input), 514579);
    }

    #[test]
    fn test_part2() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part2(&input), 241861950);
    }
}
