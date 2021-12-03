use itertools::Itertools;

pub fn raw_input() -> String {
    include_str!("../../inputs/day03.txt").to_string()
}

pub fn parse_input(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|line| u16::from_str_radix(line, 2).unwrap())
        .collect()
}

pub fn part1(input: &[u16]) -> i32 {
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    (0..16u16).for_each(|pos| {
        let bits = input.iter().map(|&int| int & (1 << pos) != 0).counts();
        if bits.len() < 2 {
            return;
        }
        let zeroes = bits.get(&false).unwrap();
        let ones = bits.get(&true).unwrap();
        if ones > zeroes {
            gamma_rate += 1 << pos;
        } else {
            epsilon_rate += 1 << pos;
        }
    });
    gamma_rate * epsilon_rate
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../inputs/day03-test.txt");

    #[test]
    fn example_part1() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 198);
    }

    #[test]
    fn test_part1() {
        let input = parse_input(&raw_input());
        assert_eq!(part1(&input), 1092896);
    }
}
