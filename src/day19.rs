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

pub fn resolve_rule(rule: &mut Rule, rulemap: &HashMap<usize, Rule>, max_len: usize) {

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
            //println!("deps: {:?}", deps);

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
            let leaves = parser(rule, rulemap, max_len);

            *rule = Rule::Leaf(leaves);

            //println!("Resolved rule: {:?}", *rule);
        },
    }
}

pub fn parser(rule: &Rule, rulemap: &HashMap<usize, Rule>, max_len: usize) -> Vec<String> {
    let mut output: Vec<String> = vec![];
    match rule {
        Rule::Node(rule_string) => {
            let re_single = Regex::new(r"^([0-9]+)$").unwrap();

            //println!("rule_string: {}", rule_string.to_string());

            for cap in re_single.captures_iter(rule_string) {
                //println!("Single regex tripped: {}", cap[0].to_string());
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
                //println!("Or regex tripped: {}", cap[0].to_string());
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
            let re_and3 = Regex::new(r"([0-9]+) ([0-9]+) ([0-9]+)").unwrap();

            let mut and3 = false;
            for cap in re_and3.captures_iter(rule_string) {
                println!("Triggered");
                and3 = true;
                let mut nums = vec![];
                nums.push(cap[1].trim_end().parse().unwrap());
                nums.push(cap[2].trim_end().parse().unwrap());
                nums.push(cap[3].trim_end().parse().unwrap());

                let mut part_vec_vec = vec![];
                for num in nums {
                    let mut part_vec = vec![];
                    let rule = rulemap.get(&num).unwrap(); // Safe because we know it's in map
                    if let Rule::Leaf(value_vec) = rule {
                        for v in value_vec {
                            let temp = v.to_string();
                            if temp.len() > max_len {
                                continue;
                            }
                            part_vec.push(temp);
                        }
                    } else {
                        panic!("Should never get here");
                    }

                    part_vec_vec.push(part_vec);
                }

                for part1 in &part_vec_vec[0] {
                    for part2 in &part_vec_vec[1] {
                        for part3 in &part_vec_vec[2] {
                            let mut tt = part1.to_string();
                            tt.push_str(part2);
                            tt.push_str(part3);
                            output.push(tt);
                        }
                    }
                }
            }

            if !and3 {
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
            }

            //println!("{:?}", output);
            return output;
        },
        Rule::Leaf(string_vec) => return string_vec.to_vec(),
    }
}

#[aoc(day19, part1)]
pub fn part1(input: &(HashMap<usize, Rule>, Vec<String>)) -> u64 {

    let mut input_clone = input.0.clone();
    let test_vec = &input.1;

    // What's the maximum length string we care about?
    let max_len = test_vec.iter().map(|x| x.len()).max().unwrap();

    for _ in 0..10 {
        let temp = input_clone.clone();
        for (k, v) in input_clone.iter_mut() {
            resolve_rule(v, &temp, max_len);
        }
        //println!("Rules: {:?}\n", input_clone);
    }

    //println!("Rule 0: {:?}\n", input_clone.get(&0).unwrap());
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

pub fn resolve_rule2(rule: &mut Rule, rulemap: &HashMap<usize, Rule>, max_len: usize) {

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
            //println!("deps: {:?}", deps);

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
            let leaves = parser(rule, rulemap, max_len);

            *rule = Rule::Leaf(leaves);

            //println!("Resolved rule: {:?}", *rule);
        },
    }
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

    // What's the maximum length string we care about?
    let max_len = test_vec.iter().map(|x| x.len()).max().unwrap();

    println!("max_len: {}", max_len);

    for _ in 0..10 {
        let temp = input_clone.clone();
        for (k, v) in input_clone.iter_mut() {
            resolve_rule2(v, &temp, max_len);
        }
        //println!("Rules: {:?}\n", input_clone);
    }

    //println!("Rule 0: {:?}\n", input_clone.get(&0).unwrap());
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
