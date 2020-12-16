use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn load_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

pub fn doit(input: &[i32], end: usize) -> i32 {
    let start = input.to_vec();
    let start_idx = start.len() + 1;
    let mut nummap: HashMap<i32, i32> = HashMap::new();
    for (i, num) in start.iter().enumerate() {
        nummap.insert(*num, i as i32 + 1);
    }
    let mut last: i32 = *input.last().unwrap();
    for turn_num in start_idx..end + 1 {
        let new_num: i32;
        if let Some(last_spoken) = nummap.get(&last) {
            new_num = turn_num as i32 - 1 - *last_spoken;
        } else {
            new_num = 0;
        }
        nummap.insert(last, turn_num as i32 - 1);
        last = new_num;
    }
    last
}

#[aoc(day15, part1)]
pub fn part1(input: &[i32]) -> i32 {
    doit(input, 2020)
}

#[aoc(day15, part2)]
pub fn part2(input: &[i32]) -> i32 {
    doit(input, 30000000)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![0, 3, 6];
        assert_eq!(part1(&input), 436);
        let input = vec![1, 3, 2];
        assert_eq!(part1(&input), 1);
        let input = vec![2, 1, 3];
        assert_eq!(part1(&input), 10);
        let input = vec![1, 2, 3];
        assert_eq!(part1(&input), 27);
    }
}
