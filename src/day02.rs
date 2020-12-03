use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug)]
pub struct Entry {
    min: usize,
    max: usize,
    letter: String,
    password: String,
}

pub fn load_input(filename: &str) -> Vec<Entry> {
    let buf = read_to_string(filename).unwrap();

    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let mut output = vec![];
    for line in buf.lines() {
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

    #[test]
    fn test_part1() {
        let x = load_input("inputs/02a.txt");
        assert_eq!(part1(&x), 2);
    }

    #[test]
    fn test_part2() {
        let x = load_input("inputs/02a.txt");
        assert_eq!(part2(&x), 1);
    }
}
