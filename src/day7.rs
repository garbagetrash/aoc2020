#![allow(clippy::map_entry)]
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Bags {
    name: String,
    num: u32,
}

#[aoc_generator(day7)]
pub fn load_input(input: &str) -> HashMap<String, Vec<Bags>> {
    let mut bagmap = HashMap::new();

    let re = Regex::new(r"^\s?([0-9]+) ([a-z\s]+) ").unwrap();
    for line in input.lines() {
        let inout: Vec<_> = line.split(" bags contain ").collect();
        let input_name = inout[0].to_string();

        let mut requirements = vec![];
        for out in inout[1].split(',') {
            for cap in re.captures_iter(out) {
                let num: u32 = cap[1].parse().unwrap();
                let bagtype = cap[2].to_string();
                requirements.push(Bags {
                    name: bagtype.to_string(),
                    num,
                });
            }
        }

        bagmap.insert(input_name, requirements);
    }
    bagmap
}

#[aoc(day7, part1)]
pub fn part1(input: &HashMap<String, Vec<Bags>>) -> usize {
    let mut bags_containing = HashSet::new();
    for (key, val) in input.iter() {
        for bag in val {
            if &bag.name == "shiny gold" {
                bags_containing.insert(key);
            }
        }
    }

    for _ in 0..4 {
        for (key, val) in input.iter() {
            for bag in val {
                if bags_containing.contains(&bag.name) {
                    bags_containing.insert(key);
                }
            }
        }
    }

    bags_containing.len()
}

#[aoc(day7, part2)]
pub fn part2(input: &HashMap<String, Vec<Bags>>) -> u32 {
    let mut bagnums = HashMap::new();

    // Insert base bags
    for (key, val) in input.iter() {
        if val.is_empty() {
            bagnums.insert(key, 1);
        }
    }

    for _ in 0..5 {
        for (key, val) in input.iter() {
            // If not already counted...
            if !bagnums.contains_key(&key) {
                let innerbags = val;
                let mut validcnt = 0;
                let mut bagcnt = 1;

                for bag in innerbags {
                    if bagnums.contains_key(&bag.name) {
                        validcnt += 1;
                        bagcnt += bag.num * bagnums.get(&bag.name).unwrap();
                    }
                }

                if validcnt == innerbags.len() {
                    bagnums.insert(key, bagcnt);
                }
            }
        }
    }

    // Grab the shiny gold bag, don't count the bag itself so remove 1
    *bagnums.get(&String::from("shiny gold")).unwrap() - 1
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/7a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 4);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/7a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 32);

        let input = read_to_string("input/7b.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 126);
    }
}
