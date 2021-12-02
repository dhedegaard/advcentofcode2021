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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

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
}
