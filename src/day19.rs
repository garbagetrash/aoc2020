use std::collections::HashMap;
use regex::Regex;


#[derive(Debug, Clone)]
pub struct Node {
    id: usize,
    used_by: Vec<usize>,
    value: String,
}

#[aoc_generator(day19)]
pub fn load_input(input: &str) -> (HashMap<usize, Node>, Vec<String>) {
    let mut rgraph = HashMap::new();
    let re = Regex::new(r"^([0-9]+): (.+)$").unwrap();

    let sections: Vec<_> = input.split("\n\n").collect();

    // First create the reverse graph with no edges
    for line in sections[0].lines() {
        for cap in re.captures_iter(line) {
            let id = cap[1].parse().unwrap();
            let rule_str = cap[2].to_string();
            let node = Node { id: id, used_by: vec![], value: rule_str.to_string() };
            rgraph.insert(id, node);
        }
    }

    // Now that all our Nodes are present and have associated IDs, lets fill
    // out those reverse edge pointers
    for (k, v) in rgraph.iter() {
        // k is an ID for a Node, scan all the Nodes (including this one!) and
        // find all references to ID k.
        for (k, v) in rgraph.iter() {
            for cap in renum.captures_iter(v.value) {
                for c in cap.iter().skip(1) {
                    deps.push(c.unwrap().as_str().parse().unwrap());
                }
            }
        }
    }

    let mut test_vec = vec![];
    for line in sections[1].lines() {
        test_vec.push(line.trim_end().to_string());
    }

    (rgraph, test_vec)
}

pub struct Magic {
    idrepr: Vec<usize>,
}

impl Magic {

    pub fn new() -> Magic {
        Magic {
            idrepr: vec![],
        }
    }

    pub fn push(&mut self, c: &char) -> Vec<usize> {
        todo!();
    }
}

#[aoc(day19, part1)]
pub fn part1(input: &(HashMap<usize, Node>, Vec<String>)) -> u64 {

    let mut input_clone = input.0.clone();
    let test_vec = &input.1;

    for line in test_vec {
        // Check each candidate
        let mut magic = Magic::new();
        for c in line.chars() {
            let idrepr = magic.push(&c);
        }
    }

    let mut cntr = 0;
    todo!()
}

#[aoc(day19, part2)]
pub fn part2(input: &(HashMap<usize, Node>, Vec<String>)) -> u64 {

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
