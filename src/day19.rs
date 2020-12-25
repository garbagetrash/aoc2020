use regex::Regex;
use std::collections::HashMap;

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
        Rule::Leaf(_) => (),
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
                    if let Rule::Node(_) = r {
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
        }
    }
}

pub fn parser(rule: &Rule, rulemap: &HashMap<usize, Rule>) -> Vec<String> {
    let mut output: Vec<String> = vec![];
    match rule {
        Rule::Node(rule_string) => {
            let re_single = Regex::new(r"^([0-9]+)$").unwrap();

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
            let re_and2 = Regex::new(r"([0-9]+) ([0-9]+)").unwrap();

            for cap in re_and2.captures_iter(rule_string) {
                let mut nums = vec![];
                nums.push(cap[1].trim_end().parse().unwrap());
                nums.push(cap[2].trim_end().parse().unwrap());

                let mut part_vec_vec = vec![];
                for num in nums {
                    let mut part_vec = vec![];
                    let rule = rulemap.get(&num).unwrap(); // Safe because we know it's in map
                    if let Rule::Leaf(value_vec) = rule {
                        for v in value_vec {
                            part_vec.push(v.to_string());
                        }
                    } else {
                        panic!("Should never get here");
                    }

                    part_vec_vec.push(part_vec);
                }

                for part1 in &part_vec_vec[0] {
                    for part2 in &part_vec_vec[1] {
                        let mut tt = part1.to_string();
                        tt.push_str(part2);
                        output.push(tt);
                    }
                }
            }
            output
        }
        Rule::Leaf(string_vec) => string_vec.to_vec(),
    }
}

pub fn is_match(input: &str, rulemap: &HashMap<usize, Rule>) -> bool {
    // Node 8 is 8: 42 | 42 8, this means 42 or 42 and 42 or 42 and 42 and 42 or...
    let rule42 = rulemap.get(&42).unwrap();

    // Node 11 is 11: 42 31 | 42 11 31, this means 42 and 31 or
    // 42 and 42 and 31 and 31 or 42 and 42 and 42 and 31 and 31 and 31 or...
    let rule31 = rulemap.get(&31).unwrap();

    if let Rule::Leaf(r31str) = rule31 {
        let mut re31str = r31str[0].to_string();
        for substr in r31str.iter().skip(1) {
            re31str = format!(r"{}|{}", re31str, substr);
        }
        let cntr31re = format!(r"({})+$", re31str);
        let rule31regex = Regex::new(&cntr31re).unwrap();

        // We care because this tells us how many times 11 recurses, and how
        // many 42's to associate with rule 11 vs rule 8
        let mut num_31s = 0;
        if let Some(caps) = rule31regex.captures(input) {
            num_31s = caps[0].len() / 8;
        }

        if num_31s < 1 {
            //return false;
        }

        if let Rule::Leaf(r42str) = rule42 {
            let mut re42str = r42str[0].to_string();
            for substr in r42str.iter().skip(1) {
                re42str = format!(r"{}|{}", re42str, substr);
            }

            // NOTE Rule 42 and Rule 31 consist of only length 8 sequences... abuse it.
            let n_chunks = input.len() / 8;
            let r31 = Regex::new(&re31str).unwrap();
            let r42 = Regex::new(&re42str).unwrap();
            let mut r42cntr = 0;
            let mut r31cntr = 0;
            let mut nomore42s = false;
            for i in 0..n_chunks {
                let mut uhoh = 0;
                let chunk = input[8 * i..8 * (i + 1)].to_string();
                if r42.captures(&chunk).is_some() {
                    r42cntr += 1;
                    uhoh += 1;
                    if nomore42s {
                        // Should get no more 42s after first 31
                        return false;
                    }
                }
                if r31.captures(&chunk).is_some() {
                    r31cntr += 1;
                    uhoh += 1;
                    nomore42s = true;
                }

                if uhoh == 2 {
                    panic!("Somehow we are rule 31 and 42, shouldn't happen");
                }
            }

            // Fail conditions
            return !(r42cntr - 1 < r31cntr || r42cntr < 2 || r31cntr < 1);
        }
    }
    false
}

#[aoc(day19, part1)]
pub fn part1(input: &(HashMap<usize, Rule>, Vec<String>)) -> u64 {
    let mut input_clone = input.0.clone();
    let test_vec = &input.1;

    for _ in 0..10 {
        let temp = input_clone.clone();
        for (_, v) in input_clone.iter_mut() {
            resolve_rule(v, &temp);
        }
    }

    let rule0 = input_clone.get(&0).unwrap();

    let mut cntr = 0;
    if let Rule::Leaf(rule0_vec) = rule0 {
        for test in test_vec {
            if rule0_vec.iter().any(|value| value == test) {
                cntr += 1;
            }
        }
    } else {
        panic!("Rule 0 didn't resolve to a Rule::Leaf() object");
    }

    cntr
}

#[aoc(day19, part2)]
pub fn part2(input: &(HashMap<usize, Rule>, Vec<String>)) -> u64 {
    let mut input_clone = input.0.clone();
    let test_vec = &input.1;

    // Modify input as specified
    if let Some(value) = input_clone.get_mut(&8) {
        *value = Rule::Node("42 | 42 8".to_string());
    }
    if let Some(value) = input_clone.get_mut(&11) {
        *value = Rule::Node("42 31 | 42 11 31".to_string());
    }

    for _ in 0..10 {
        let temp = input_clone.clone();
        for (_k, v) in input_clone.iter_mut() {
            resolve_rule(v, &temp);
        }
    }

    // NOTE: These are really just the last two nodes prior to node 0... let's
    // just cheat and special case them because screw it.

    let mut cntr = 0;
    for test in test_vec {
        if is_match(test, &input_clone) {
            cntr += 1;
        }
    }
    cntr
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
        let input = read_to_string("input/19c.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 12);
    }
}
