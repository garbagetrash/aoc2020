use num::integer::lcm;

#[derive(Copy, Clone, Debug)]
pub struct Schedule {
    offset: i64,
    cycle: i64,
}

#[aoc_generator(day13)]
pub fn load_input(input: &str) -> (i64, Vec<Schedule>) {
    let lines: Vec<_> = input.lines().collect();
    let earliest: i64 = lines[0].parse().unwrap();
    let mut buses = vec![];
    for id in lines[1].split(',') {
        buses.push(String::from(id));
    }
    let mut scheds = vec![];
    for (i, id) in buses.iter().enumerate() {
        if id != "x" {
            let value = id.parse().unwrap();
            scheds.push(Schedule {
                offset: i as i64,
                cycle: value,
            });
        }
    }
    (earliest, scheds)
}

#[aoc(day13, part1)]
pub fn part1(input: &(i64, Vec<Schedule>)) -> i64 {
    let mut time = input.0;
    let buses = &input.1;
    loop {
        for id in buses {
            if time % id.cycle == 0 {
                return id.cycle * (time - input.0);
            }
        }

        time += 1;
    }
}

pub fn roll_schedules(base: &Schedule, new: &Schedule) -> Schedule {
    let mut time = base.offset;
    loop {
        if (time + new.offset) % new.cycle == 0 {
            return Schedule {
                offset: time,
                cycle: lcm(base.cycle, new.cycle),
            };
        } else {
            time += base.cycle;
        }
    }
}

#[aoc(day13, part2)]
pub fn part2(input: &(i64, Vec<Schedule>)) -> i64 {
    let ans = input.1.iter().fold(
        Schedule {
            offset: 0,
            cycle: 1,
        },
        |acc, x| roll_schedules(&acc, &x),
    );

    ans.offset
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/13a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 295);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/13a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 1068781);
        let input = read_to_string("input/13b.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 3417);
        let input = read_to_string("input/13c.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 754018);
        let input = read_to_string("input/13d.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 779210);
        let input = read_to_string("input/13e.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 1261476);
        let input = read_to_string("input/13f.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 1202161486);
    }
}
