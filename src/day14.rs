use regex::Regex;
use std::collections::HashMap;
use std::iter;

#[derive(Debug)]
pub enum Instruction {
    Mask(String),
    Mem((i64, i64)),
}

#[aoc_generator(day14)]
pub fn load_input(input: &str) -> Vec<Instruction> {
    let remask = Regex::new(r"^(\S+) = ([0-9,X]+)$").unwrap();
    let remem = Regex::new(r"^mem\[([0-9]+)\]").unwrap();
    let mut output = vec![];
    for line in input.lines() {
        for cap in remask.captures_iter(line) {
            match &cap[1][..3] {
                "mem" => {
                    let mut loc = 0;
                    for c in remem.captures_iter(&cap[1]) {
                        loc = c[1].parse().unwrap();
                    }
                    let value: i64 = cap[2].parse().unwrap();
                    output.push(Instruction::Mem((loc, value)));
                }
                "mas" => {
                    let mask = String::from(cap[2].to_string());
                    output.push(Instruction::Mask(mask));
                }
                _ => (),
            }
        }
    }
    output
}

#[aoc(day14, part1)]
pub fn part1(input: &[Instruction]) -> i64 {
    let mut mask = String::from("000000000000000000000000000000000000");
    let mut mem: HashMap<i64, i64> = HashMap::new();
    for inst in input {
        match inst {
            Instruction::Mask(m) => {
                mask = m.clone();
            }
            Instruction::Mem((loc, value)) => {
                let strrep = format!("{:036b}", value);
                let mut newstr = String::new();
                for (s, m) in strrep.chars().zip(mask.chars()) {
                    match m {
                        'X' => newstr.push(s),
                        '0' => newstr.push('0'),
                        '1' => newstr.push('1'),
                        _ => (),
                    }
                }
                let newvalue = i64::from_str_radix(&newstr, 2).unwrap();
                mem.insert(*loc, newvalue);
            }
        }
    }

    let mut sum_values = 0;
    for val in mem.values() {
        sum_values += val;
    }
    sum_values
}

#[aoc(day14, part2)]
pub fn part2(input: &[Instruction]) -> i64 {
    let mut mask = String::from("000000000000000000000000000000000000");
    let mut mem: HashMap<i64, i64> = HashMap::new();
    for inst in input {
        match inst {
            Instruction::Mask(m) => {
                mask = m.clone();
            }
            Instruction::Mem((loc, value)) => {
                let strrep = format!("{:036b}", loc);
                let mut newstr = String::new();
                for (s, m) in strrep.chars().zip(mask.chars()) {
                    match m {
                        'X' => newstr.push('X'),
                        '0' => newstr.push(s),
                        '1' => newstr.push('1'),
                        _ => (),
                    }
                }

                // This painful block is about creating the valid floating addresses
                // First we have to create list of binary numbers to fill X's with.
                // This whole section would be 1 line if rust's format!() simply allowed
                // variable length zero padding, but as far as I can tell it doesn't.
                let float_cnt = newstr.matches('X').count();
                let mut addr_list = vec![];
                let mut addrs: Vec<String> = vec![];
                for num in 0..2_i64.pow(float_cnt as u32) {
                    let numstr = format!("{:b}", num);
                    let nslen = float_cnt - numstr.chars().count();
                    let new_addr = iter::repeat('0').take(nslen).chain(numstr.chars()).collect();
                    addrs.push(new_addr);
                }

                for a in addrs {
                    let mut cnt = 0;
                    let mut temp = String::new();
                    for c in newstr.chars() {
                        match c {
                            'X' => {
                                temp.push_str(&a.chars().nth(cnt).unwrap().to_string());
                                cnt += 1;
                            }
                            _ => temp.push(c),
                        }
                    }
                    addr_list.push(i64::from_str_radix(&temp, 2).unwrap());
                }

                // Finally we actually populate all the addresses in addr_list
                for a in addr_list {
                    mem.insert(a, *value);
                }
            }
        }
    }

    let mut sum_values = 0;
    for val in mem.values() {
        sum_values += val;
    }
    sum_values
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/14a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 165);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/14b.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 208);
    }
}
