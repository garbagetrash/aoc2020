use regex::Regex;

#[derive(Debug)]
pub struct Entry {
    min: usize,
    max: usize,
    letter: String,
    password: String,
}

#[aoc_generator(day2)]
pub fn load_input(input: &str) -> Vec<Entry> {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let mut output = vec![];
    for line in input.lines() {
        for cap in re.captures_iter(line) {
            let new = Entry {
                min: cap[1].parse().unwrap(),
                max: cap[2].parse().unwrap(),
                letter: cap[3].to_string(),
                password: cap[4].to_string(),
            };
            output.push(new);
        }
    }

    output
}

#[aoc(day2, part1)]
pub fn part1(input: &[Entry]) -> u64 {
    let mut valid = 0;
    for e in input {
        let mut cntr = 0;
        for c in e.password.chars() {
            if c.to_string() == e.letter {
                cntr += 1;
            }
        }

        if cntr >= e.min && cntr <= e.max {
            valid += 1;
        }
    }

    valid
}

#[aoc(day2, part2)]
pub fn part2(input: &[Entry]) -> u64 {
    let mut valid = 0;
    for e in input {
        let mut cntr = 0;

        if e.password.chars().nth(e.min - 1).unwrap().to_string() == e.letter {
            cntr += 1;
        }

        if e.password.chars().nth(e.max - 1).unwrap().to_string() == e.letter {
            cntr += 1;
        }

        if cntr == 1 {
            valid += 1;
        }
    }

    valid
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/02a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/02a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 1);
    }
}
