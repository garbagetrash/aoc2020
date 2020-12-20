use std::collections::HashMap;
use regex::Regex;


#[derive(Debug, Clone)]
pub enum Rule {
    Leaf(Vec<String>),
    Node(String),
}

#[aoc_generator(day19)]
pub fn load_input(input: &str) -> (HashMap<usize, Rule>, Vec<String>) {
    let mut rules = HashMap::new();
    let re = Regex::new(r"^([0-9]+): (.+)$").unwrap();
    let leafre = Regex::new(r#""([ab])""#).unwrap();

    let sections: Vec<_> = input.split("\n\n").collect();

    for line in sections[0].lines() {
        for cap in re.captures_iter(line) {
            let id = cap[1].parse().unwrap();
            let rule_str = cap[2].to_string();
            if let Some(m) = leafre.captures(&rule_str) {
                rules.insert(id, Rule::Leaf(vec![m[1].to_string()]));
            } else {
                rules.insert(id, Rule::Node(rule_str));
            }
        }
    }

    let mut test_vec = vec![];
    for line in sections[1].lines() {
        test_vec.push(line.trim_end().to_string());
    }

    (rules, test_vec)
}

pub fn resolve_rule(rule: &mut Rule, rulemap: &HashMap<usize, Rule>) {

    match rule {
        Rule::Leaf(value) => (),
        Rule::Node(value) => {
            let numsre = Regex::new(r"([0-9]+)").unwrap();
            let mut deps: Vec<usize> = vec![];
            for cap in numsre.captures_iter(value) {
                for c in cap.iter().skip(1) {
                    deps.push(c.unwrap().as_str().parse().unwrap());
                }
            }

            // If we have all deps, resolve them and calc a new Rule::Leaf!...
            // else just leave it alone for later.
            for dep in &deps {
                if let Some(r) = rulemap.get(dep) {
                    if let Rule::Node(x) = r {
                        // Bail if we can't resolve yet
                        return;
                    }
                } else {
                    // Rule not in hashmap, obviously can't resolve
                    // This should never happen btw
                    panic!("Requested dependency not in rulemap");
                }
            }

            // If we get here then we can resolve the Leaf
            let leaves = parser(rule, rulemap);

            *rule = Rule::Leaf(leaves);
        },
    }
}

pub fn parser(rule: &Rule, rulemap: &HashMap<usize, Rule>) -> Vec<String> {
    match rule {
        Rule::Node(rule_string) => {
            let re_single = Regex::new(r"^([0-9]+)$").unwrap();

            let mut output: Vec<String> = vec![];

            for cap in re_single.captures_iter(rule_string) {
                let num: usize = cap[1].parse().unwrap();
                let rule = rulemap.get(&num).unwrap(); // Safe because we know it's in map
                if let Rule::Leaf(value_vec) = rule {
                    for v in value_vec {
                        output.push(v.to_string());
                    }
                } else {
                    panic!("Should never get here");
                }

                // Ok to bail here, if we matched it was just this number
                return output;
            }


            // Handle single or no ands
            let re_or = Regex::new(r"^([0-9]+) \| ([0-9]+)$").unwrap();
            for cap in re_or.captures_iter(rule_string) {
                let num1: usize = cap[1].parse().unwrap();
                let num2: usize = cap[2].parse().unwrap();

                let rule = rulemap.get(&num1).unwrap(); // Safe because we know it's in map
                if let Rule::Leaf(value_vec) = rule {
                    for v in value_vec {
                        output.push(v.to_string());
                    }
                } else {
                    panic!("Should never get here");
                }

                let rule = rulemap.get(&num2).unwrap(); // Safe because we know it's in map
                if let Rule::Leaf(value_vec) = rule {
                    for v in value_vec {
                        output.push(v.to_string());
                    }
                } else {
                    panic!("Should never get here");
                }

                // Ok to bail here, if we matched it was just this or
                return output;
            }

            // Resolve ands, return strings
            let re_and = Regex::new(r"([0-9]+) ([0-9]+)").unwrap();
            for cap in re_and.captures_iter(rule_string) {
                let num1: usize = cap[1].parse().unwrap();
                let num2: usize = cap[2].parse().unwrap();

                let mut part1_vec = vec![];
                let rule = rulemap.get(&num1).unwrap(); // Safe because we know it's in map
                if let Rule::Leaf(value_vec) = rule {
                    for v in value_vec {
                        part1_vec.push(v.to_string());
                    }
                } else {
                    panic!("Should never get here");
                }

                let mut part2_vec = vec![];
                let rule = rulemap.get(&num2).unwrap(); // Safe because we know it's in map
                if let Rule::Leaf(value_vec) = rule {
                    for v in value_vec {
                        part2_vec.push(v.to_string());
                    }
                } else {
                    panic!("Should never get here");
                }

                for part1 in part1_vec {
                    for part2 in &part2_vec {
                        let mut tt = part1.to_string();
                        tt.push_str(part2);
                        output.push(tt);
                    }
                }
            }

            return output;
        },
        Rule::Leaf(string_vec) => return string_vec.to_vec(),
    }
}

#[aoc(day19, part1)]
pub fn part1(input: &(HashMap<usize, Rule>, Vec<String>)) -> u64 {

    let mut input_clone = input.0.clone();
    let test_vec = &input.1;

    for _ in 0..10 {
        let temp = input_clone.clone();
        for (k, v) in input_clone.iter_mut() {
            resolve_rule(v, &temp);
        }
    }

    let rule0 = input_clone.get(&0).unwrap();

    let mut cntr = 0;
    if let Rule::Leaf(rule0_vec) = rule0 {
        for test in test_vec {
            if let Some(_) = rule0_vec.iter().filter(|&value| value == test).next() {
                cntr += 1;
            }
        }
    } else {
        panic!("Rule 0 didn't resolve to a Rule::Leaf() object");
    }

    cntr
}

pub struct ReverseGraph {
    id: usize,
    used_by: Vec<usize>,
    value: String,
}

#[aoc(day19, part2)]
pub fn part2(input: &(HashMap<usize, Rule>, Vec<String>)) -> u64 {

    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/19a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 2);

        let input = read_to_string("input/19b.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/19b.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 0);
    }
}
