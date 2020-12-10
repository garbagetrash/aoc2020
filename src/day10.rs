#[aoc_generator(day10)]
pub fn load_input(input: &str) -> Vec<u64> {
    let mut output = vec![];
    for line in input.lines() {
        output.push(line.parse().unwrap());
    }
    output
}

#[aoc(day10, part1)]
pub fn part1(input: &[u64]) -> u64 {
    let mut inclone = input.to_vec();
    inclone.sort_unstable();
    let device_jolt: u64 = inclone.last().unwrap() + 3;
    inclone.push(device_jolt);

    let mut last = 0;
    let mut one_count = 0;
    let mut three_count = 0;
    for d in inclone {
        if d - last == 1 {
            one_count += 1;
        } else if d - last == 3 {
            three_count += 1;
        }
        last = d;
    }
    one_count * three_count
}

/* LOL
3=
0 3 - 1

0s 1

1 3=
0 1 4 - 1

0s 1

1 1 3=
0 1 2 5 - 2

0s 1
1s 1

1 1 1 3=
0 1 2 3 6 - 4

0s 1
1s 2
2s 1

1 1 1 1 3=
0 1 2 3 4 7 - 7

0s 1
1s 3
2s 3
*/
#[aoc(day10, part2)]
pub fn part2(input: &[u64]) -> u64 {
    let mut inclone = input.to_vec();
    inclone.sort_unstable();
    inclone.insert(0, 0);
    let device_jolt: u64 = inclone.last().unwrap() + 3;
    inclone.push(device_jolt);

    let mut diffs = vec![];
    for i in 0..inclone.len() - 1 {
        diffs.push(inclone[i+1] - inclone[i]);
    }

    let asdf = diffs.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    let fdsa = asdf.join("");
    let asdf = fdsa.split('3').collect::<Vec<_>>();

    let mut acc = 1;
    for b in asdf {
        match b {
            "11" => acc *= 2,
            "111" => acc *= 4,
            "1111" => acc *= 7,
            _ => (),
        }
    }

    acc
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/10a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 7 * 5);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/10a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 8);

        let input = read_to_string("input/10b.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 19208);
    }
}
