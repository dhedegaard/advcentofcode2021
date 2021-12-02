pub fn raw_input() -> String {
    include_str!("../../inputs/day01.txt").to_string()
}

pub fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

pub fn part1(input: &[i32]) -> i32 {
    input.iter().enumerate().fold(0, |acc, (index, &item)| {
        if index > 0 && input[index - 1] < item {
            acc + 1
        } else {
            acc
        }
    })
}

pub fn part2(input: &[i32]) -> i32 {
    let input = input
        .iter()
        .enumerate()
        .map(|(index, &elem)| {
            if index + 2 < input.len() {
                elem + input[index + 1] + input[index + 2]
            } else {
                0
            }
        })
        .filter(|&e| e != 0)
        .collect::<Vec<_>>();
    input.iter().enumerate().fold(0, |acc, (index, &item)| {
        if item != 0 && index >= 1 && index > 0 && input[index - 1] < item {
            acc + 1
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const TEST_INPUT: &str = include_str!("../../inputs/day01-test.txt");

    #[test]
    fn examples_part1() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn test_part() {
        let input = parse_input(&raw_input());
        assert_eq!(part1(&input), 1288);
    }

    #[test]
    fn examples_part2() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part2(&input), 5);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(&raw_input());
        assert_eq!(part2(&input), 1311);
    }
}
