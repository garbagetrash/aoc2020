#[aoc_generator(day5)]
pub fn load_input(input: &str) -> Vec<u32> {
    let mut output = vec![];
    for line in input.lines() {
        let mut row: u32 = 0;
        let mut col: u32 = 0;
        for c in line.chars() {
            match c {
                'F' => {
                    row <<= 1;
                }
                'B' => {
                    row <<= 1;
                    row += 1;
                }
                'L' => {
                    col <<= 1;
                }
                'R' => {
                    col <<= 1;
                    col += 1;
                }
                _ => (),
            }
        }

        let id = row * 8 + col;
        output.push(id);
    }
    output
}

#[aoc(day5, part1)]
pub fn part1(input: &[u32]) -> u32 {
    *input.iter().max().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &[u32]) -> u32 {
    let mut input_copy = input.clone().to_vec();
    input_copy.sort_unstable();

    for i in *input_copy.first().unwrap()..*input_copy.last().unwrap() {
        if !(input_copy.iter().any(|x| *x == i)) {
            return i;
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = load_input("FBFBBFFRLR");
        assert_eq!(part1(&input), 357);

        let input = load_input("BFFFBBFRRR");
        assert_eq!(part1(&input), 567);

        let input = load_input("FFFBBBFRRR");
        assert_eq!(part1(&input), 119);

        let input = load_input("BBFFBBFRLL");
        assert_eq!(part1(&input), 820);
    }
}
