pub fn load_input() -> Vec<u64> {
    vec![0, 0]
}

pub fn part1(input: &Vec<u64>) -> u64 {
    0
}

pub fn part2(input: &Vec<u64>) -> u64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(fuel_required(12), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(fuel_required_recursive(14), 0);
    }
}
