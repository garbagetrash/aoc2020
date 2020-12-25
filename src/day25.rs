#[aoc_generator(day25)]
pub fn load_input(input: &str) -> Vec<u64> {
    let mut output = vec![];
    for line in input.lines() {
        output.push(line.parse().unwrap());
    }
    output
}

pub fn transform(value: u64, subj_num: u64) -> u64 {
    (value * subj_num) % 20201227
}

pub fn transform_n_times(subj_num: u64, n_times: u64) -> u64 {
    let mut value = 1;
    for _ in 0..n_times {
        value = transform(value, subj_num);
    }
    value
}

pub fn find_loop_size(subj_num: u64, pub_key: u64) -> u64 {
    let mut value = 1;
    let mut loop_size = 0;
    loop {
        value = transform(value, subj_num);

        loop_size += 1;
        if value == pub_key {
            return loop_size;
        }
    }
}

#[aoc(day25, part1)]
pub fn part1(input: &[u64]) -> u64 {
    //let card_size = find_loop_size(7, input[0]);
    let door_size = find_loop_size(7, input[1]);

    transform_n_times(input[0], door_size)
    //let output2 = transform_n_times(input[1], card_size);
}

#[aoc(day25, part2)]
pub fn part2(_input: &[u64]) -> u64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/25a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 14897079);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/25a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 0);
    }
}
