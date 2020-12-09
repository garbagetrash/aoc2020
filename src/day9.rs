use std::cmp::Ordering;

#[aoc_generator(day9)]
pub fn load_input(input: &str) -> Vec<u64> {
    let mut output = vec![];
    for line in input.lines() {
        output.push(line.parse().unwrap());
    }
    output
}

pub fn is_valid(innum: u64, input: &[u64], n_nums: usize) -> bool {
    for i in 0..n_nums {
        for j in (i + 1)..n_nums {
            if input[i] + input[j] == innum {
                return true;
            }
        }
    }

    false
}

pub fn actual_code1(input: &[u64], cnt: usize) -> u64 {
    for i in 0..input.len() {
        let innum = input[i + cnt];
        let slice = &input[i..i + cnt];
        if !is_valid(innum, &slice, cnt) {
            return innum;
        }
    }

    // should never get here
    0
}

#[aoc(day9, part1)]
pub fn part1(input: &[u64]) -> u64 {
    actual_code1(input, 25)
}

pub fn actual_code2(input: &[u64], cnt: usize) -> u64 {
    let num = actual_code1(input, cnt);

    for i in 0..input.len() {
        let mut n_nums = 1;
        loop {
            let thesum = input[i..i + n_nums].iter().sum::<u64>();
            match thesum.cmp(&num) {

                Ordering::Equal => {
                    let thelist = input[i..i + n_nums].to_vec();
                    let smallest = thelist.iter().min().unwrap();
                    let largest = thelist.iter().max().unwrap();
                    return smallest + largest;
                },
                Ordering::Less => n_nums += 1,
                _ => break,
            }
        }
    }

    // should never get here
    0
}

#[aoc(day9, part2)]
pub fn part2(input: &[u64]) -> u64 {
    actual_code2(input, 25)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/9a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(actual_code1(&input, 5), 127);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/9a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(actual_code2(&input, 5), 62);
    }
}
