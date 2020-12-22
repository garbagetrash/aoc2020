use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Node {
    id: usize,
    used_by: HashSet<usize>,
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
            let node = Node {
                id: id,
                used_by: HashSet::new(),
                value: rule_str.trim_matches('\"').to_string(),
            };
            rgraph.insert(id, node);
        }
    }

    // Now that all our Nodes are present and have associated IDs, lets fill
    // out those reverse edge pointers
    let rgraph_clone = rgraph.clone();
    let renum = Regex::new(r"([0-9]+)").unwrap();
    for (k, v) in rgraph.iter_mut() {
        // k is an ID for a Node, scan all the Nodes (including this one!) and
        // find all references to ID k.
        for (k2, v2) in rgraph_clone.iter() {
            for cap in renum.captures_iter(&v2.value) {
                // Get k2 node dependencies to scan for k1
                let mut deps = vec![];
                for c in cap.iter().skip(1) {
                    deps.push(c.unwrap().as_str().parse().unwrap());
                }

                // Make the reverse connection if k2 node contains k1 node as a
                // dependency
                if deps.contains(k) {
                    v.used_by.insert(*k2);
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
    rgraph: HashMap<usize, Node>,
}

impl Magic {
    pub fn new(rgraph: HashMap<usize, Node>) -> Magic {
        Magic {
            idrepr: vec![],
            rgraph,
        }
    }

    pub fn push(&mut self, c: &char) {
        for (id, node) in self.rgraph.iter() {
            if node.value == c.to_string() {
                self.idrepr.push(*id);
            }
        }
    }

    pub fn resolve(&mut self) -> bool {
        let mut cand = self.idrepr.clone();
        todo!()
    }
}

#[aoc(day19, part1)]
pub fn part1(input: &(HashMap<usize, Node>, Vec<String>)) -> u64 {
    let test_vec = &input.1;

    let mut cntr = 0;
    for line in test_vec {
        // Check each candidate
        let mut magic = Magic::new(input.0.clone());
        for c in line.chars() {
            let idrepr = magic.push(&c);
        }

        println!("test string: {}", line);
        println!("idrepr: {:?}\n", magic.idrepr);

        if magic.resolve() {
            // We have a match
            cntr += 1;
        }
    }

    cntr
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
