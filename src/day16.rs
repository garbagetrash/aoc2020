use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Field {
    field: String,
    l0: i64,
    h0: i64,
    l1: i64,
    h1: i64,
}

#[aoc_generator(day16)]
pub fn load_input(input: &str) -> (Vec<Field>, Vec<i64>, Vec<Vec<i64>>) {
    let revalid = Regex::new(r"^(.+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)$").unwrap();

    let mut field_ranges = vec![];
    let sections: Vec<_> = input.split("\n\n").collect();
    for line in sections[0].lines() {
        for cap in revalid.captures_iter(line) {
            let field = cap[1].to_string();
            let l0: i64 = cap[2].parse().unwrap();
            let h0: i64 = cap[3].parse().unwrap();
            let l1: i64 = cap[4].parse().unwrap();
            let h1: i64 = cap[5].parse().unwrap();

            field_ranges.push(Field {
                field,
                l0,
                h0,
                l1,
                h1,
            });
        }
    }

    let my_ticket: Vec<i64> = sections[1].lines().collect::<Vec<_>>()[1]
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let other_tickets: Vec<_> = sections[2].lines().skip(1).collect::<Vec<_>>();
    let mut new_ot: Vec<Vec<i64>> = vec![];
    for line in other_tickets {
        new_ot.push(line.split(',').map(|x| x.parse::<i64>().unwrap()).collect());
    }

    (field_ranges, my_ticket, new_ot)
}

#[aoc(day16, part1)]
pub fn part1(input: &(Vec<Field>, Vec<i64>, Vec<Vec<i64>>)) -> i64 {
    let other_tickets = input.2.clone();
    let fields = input.0.clone();
    let mut invalid_values = vec![];
    for ticket in other_tickets {
        for value in ticket {
            let mut valid = false;
            for field in &fields {
                if (value >= field.l0 && value <= field.h0)
                    || (value >= field.l1 && value <= field.h1)
                {
                    valid = true;
                }
            }
            if !valid {
                invalid_values.push(value);
            }
        }
    }
    invalid_values.iter().sum::<i64>()
}

#[aoc(day16, part2)]
pub fn part2(input: &(Vec<Field>, Vec<i64>, Vec<Vec<i64>>)) -> i64 {
    let other_tickets = input.2.clone();
    let fields = input.0.clone();
    let mut valid_tickets = vec![];
    for ticket in other_tickets {
        let mut ticket_valid = true;
        for value in &ticket {
            let mut valid = false;
            for field in &fields {
                if (value >= &field.l0 && value <= &field.h0)
                    || (value >= &field.l1 && value <= &field.h1)
                {
                    valid = true;
                }
            }
            if !valid {
                ticket_valid = false;
            }
        }

        if ticket_valid {
            valid_tickets.push(ticket.clone());
        }
    }

    // Now we have valid_tickets
    let n_fields = fields.len();
    let mut field_map: HashMap<String, Vec<i64>> = HashMap::new();
    for field in fields {
        let mut possible_is = vec![];
        for i in 0..n_fields {
            let mut all_valid = true;
            for ticket in &valid_tickets {
                let value = ticket[i];
                if (value >= field.l0 && value <= field.h0)
                    || (value >= field.l1 && value <= field.h1)
                {
                    //derp
                } else {
                    all_valid = false;
                    break;
                }
            }
            if all_valid {
                // Looks like index i in tickets could refer to field
                possible_is.push(i as i64);
            }
        }
        field_map.insert(field.field, possible_is);
    }

    let mut final_map: HashMap<String, i64> = HashMap::new();
    for _ in 0..n_fields {
        let mut temp_map: HashMap<String, Vec<i64>> = field_map.clone();
        for (k, v) in field_map {
            if v.len() == 1 {
                // uniquely determined!
                let ivalue = *v.first().unwrap();
                final_map.insert(k.clone(), ivalue);

                for (_k, v) in temp_map.iter_mut() {
                    *v = (*v)
                        .iter()
                        .filter(|&&x| x != ivalue)
                        .copied()
                        .collect::<Vec<i64>>()
                        .to_vec();
                }

                break;
            }
        }
        field_map = temp_map
    }

    let my_ticket = input.1.clone();
    let mut nums: Vec<i64> = vec![];
    for (k, v) in final_map.iter() {
        if k.contains("departure") {
            nums.push(my_ticket[*v as usize]);
        }
    }

    nums.iter().product::<i64>()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/16a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 71);
    }
}
