use regex::Regex;

#[aoc_generator(day18)]
pub fn load_input(input: &str) -> Vec<String> {
    let mut output = vec![];
    for line in input.lines() {
        output.push(line.to_string());
    }
    output
}

pub fn ops_eval(input: &str) -> i64 {
    let opsvec: Vec<_> = input.split_whitespace().collect();
    let start: i64 = opsvec[0].parse().unwrap();

    opsvec[1..].chunks(2).fold(start, |acc, tup| {
        let mut output = 0;
        match tup[0] {
            "*" => {
                output = acc * tup[1].parse::<i64>().unwrap();
            }
            "+" => {
                output = acc + tup[1].parse::<i64>().unwrap();
            }
            _ => (),
        }
        output
    })
}

pub fn step(input: &str) -> String {
    let re = Regex::new(r"\([^\(\)]+\)").unwrap();
    let mut input_clone = input.to_string();

    loop {
        if let Some(m) = re.find(&input_clone) {
            let thestr: String = m.as_str().to_string();
            let mut thestrclone = thestr.clone();
            thestrclone.remove(0);
            thestrclone.pop();
            let inner_value: String = ops_eval(&thestrclone).to_string();
            input_clone = input_clone.replace(&thestr, &inner_value);
        } else {
            return input_clone.to_string();
        }
    }
}

#[aoc(day18, part1)]
pub fn part1(input: &[String]) -> i64 {
    let mut nums = vec![];
    for line in input {
        let asdf = step(&line);
        nums.push(ops_eval(&asdf));
    }
    nums.iter().sum::<i64>()
}

pub fn step2(input: &str) -> String {
    let re = Regex::new(r"\([^\(\)]+\)").unwrap();
    let mut input_clone = input.to_string();

    loop {
        if let Some(m) = re.find(&input_clone) {
            let thestr: String = m.as_str().to_string();
            let mut thestrclone = thestr.clone();
            thestrclone.remove(0);
            thestrclone.pop();
            let inner_value: String = ops_eval2(&thestrclone).to_string();
            input_clone = input_clone.replace(&thestr, &inner_value);
        } else {
            return input_clone.to_string();
        }
    }
}

pub fn ops_eval2(input: &str) -> i64 {
    let re = Regex::new(r"([0-9]+)\s\+\s([0-9]+)").unwrap();
    let mut input_clone = input.to_string();

    while let Some(cap) = re.captures(&input_clone) {
        let thestr: String = cap.get(0).unwrap().as_str().to_string();
        let num1: i64 = cap[1].parse().unwrap();
        let num2: i64 = cap[2].parse().unwrap();
        let inner_value: String = (num1 + num2).to_string();
        input_clone = input_clone.replace(&thestr, &inner_value);
    }

    ops_eval(&input_clone)
}

#[aoc(day18, part2)]
pub fn part2(input: &[String]) -> i64 {
    let mut nums = vec![];
    for line in input {
        let asdf = step2(&line);
        nums.push(ops_eval2(&asdf));
    }
    nums.iter().sum::<i64>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec!["1 + 2 * 3 + 4 * 5 + 6".to_string()];
        assert_eq!(part1(&input), 71);
        let input = vec!["1 + (2 * 3) + (4 * (5 + 6))".to_string()];
        assert_eq!(part1(&input), 51);
        let input = vec!["2 * 3 + (4 * 5)".to_string()];
        assert_eq!(part1(&input), 26);
        let input = vec!["5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()];
        assert_eq!(part1(&input), 437);
        let input = vec!["5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()];
        assert_eq!(part1(&input), 12240);
        let input = vec!["((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()];
        assert_eq!(part1(&input), 13632);
    }

    #[test]
    fn test_part2() {
        let input = vec!["1 + 2 * 3 + 4 * 5 + 6".to_string()];
        assert_eq!(part2(&input), 231);
        let input = vec!["1 + (2 * 3) + (4 * (5 + 6))".to_string()];
        assert_eq!(part2(&input), 51);
        let input = vec!["2 * 3 + (4 * 5)".to_string()];
        assert_eq!(part2(&input), 46);
        let input = vec!["5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()];
        assert_eq!(part2(&input), 1445);
        let input = vec!["5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()];
        assert_eq!(part2(&input), 669060);
        let input = vec!["((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()];
        assert_eq!(part2(&input), 23340);
    }
}
