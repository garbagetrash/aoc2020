use regex::Regex;
use std::collections::HashMap;

pub struct ID {
    byr: u32,
    iyr: u32,
    eyr: u32,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

#[aoc_generator(day4)]
pub fn load_input(input: &str) -> Vec<ID> {
    let mut output = vec![];

    let pports: Vec<_> = input.split("\n\n").collect();
    let re = Regex::new(r"([a-z]+):([a-z0-9#]+)").unwrap();
    let mut pport: HashMap<String, String> = HashMap::new();
    for pp in pports {
        pport.clear();
        let iter = pp.split_whitespace();
        for field in iter {
            for cap in re.captures_iter(field) {
                pport.insert(cap[1].to_string(), cap[2].to_string());
            }
        }

        // New passport
        if !pport.contains_key("byr") {
            continue;
        }

        if !pport.contains_key("iyr") {
            continue;
        }

        if !pport.contains_key("eyr") {
            continue;
        }

        if !pport.contains_key("hgt") {
            continue;
        }

        if !pport.contains_key("hcl") {
            continue;
        }

        if !pport.contains_key("ecl") {
            continue;
        }

        if !pport.contains_key("pid") {
            continue;
        }

        // Valid passport!
        let mut id = ID {
            byr: pport["byr"].parse().unwrap(),
            iyr: pport["iyr"].parse().unwrap(),
            eyr: pport["eyr"].parse().unwrap(),
            hgt: pport["hgt"].clone(),
            hcl: pport["hcl"].clone(),
            ecl: pport["ecl"].clone(),
            pid: pport["pid"].clone(),
            cid: None,
        };

        if pport.contains_key("cid") {
            id.cid = Some(pport["cid"].clone());
        }

        output.push(id);
    }
    output
}

#[aoc(day4, part1)]
pub fn part1(input: &[ID]) -> usize {
    input.len()
}

#[aoc(day4, part2)]
pub fn part2(input: &[ID]) -> usize {
    // I hate regex.
    let hgtre = Regex::new(r"([0-9]+)(in|cm)").unwrap();
    let hclre = Regex::new(r"#([0-9a-f]{6})").unwrap();
    let pidre = Regex::new(r"^([0-9]{9})$").unwrap();

    let mut cntr = 0;
    for id in input {
        let mut valid = true;

        // BYR
        if id.byr < 1920 || id.byr > 2002 {
            valid = false;
        }

        // IYR
        if id.iyr < 2010 || id.iyr > 2020 {
            valid = false;
        }

        // EYR
        if id.eyr < 2020 || id.eyr > 2030 {
            valid = false;
        }

        // HGT
        if !hgtre.is_match(&id.hgt) {
            valid = false;
        } else if let Some(cap) = hgtre.captures(&id.hgt) {
            let value: u32 = cap[1].parse().unwrap();
            let t = cap[2].to_string();

            if &t == "cm" {
                if value < 150 || value > 193 {
                    valid = false;
                }
            } else if value < 59 || value > 76 {
                valid = false;
            }
        }

        // HCL
        if !hclre.is_match(&id.hcl) {
            valid = false;
        }

        // ECL
        if !(id.ecl == "amb"
            || id.ecl == "blu"
            || id.ecl == "brn"
            || id.ecl == "gry"
            || id.ecl == "grn"
            || id.ecl == "hzl"
            || id.ecl == "oth")
        {
            valid = false;
        }

        // PID
        if !pidre.is_match(&id.pid) {
            valid = false;
        }

        if valid {
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
        let input = read_to_string("input/04a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 2);
    }
}
