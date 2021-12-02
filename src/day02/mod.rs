#[derive(Debug)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug)]
pub struct Instruction {
    direction: Direction,
    unit: i32,
}

pub fn raw_input() -> String {
    include_str!("../../inputs/day02.txt").to_string()
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace().take(2);
            Instruction {
                direction: match parts.next().unwrap() {
                    "forward" => Direction::Forward,
                    "down" => Direction::Down,
                    "up" => Direction::Up,
                    "R" => Direction::Forward,
                    direction => panic!("Unknown direction: {}", direction),
                },
                unit: parts.next().unwrap().parse::<i32>().unwrap(),
            }
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: &[Instruction]) -> i32 {
    let mut position = 0;
    let mut depth = 0;
    for instruction in input {
        match instruction.direction {
            Direction::Forward => position += instruction.unit,
            Direction::Down => depth += instruction.unit,
            Direction::Up => depth -= instruction.unit,
        }
    }
    position * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../inputs/day02-test.txt");

    #[test]
    fn examples_part1() {
        let input = parse_input(&TEST_INPUT);
        assert_eq!(part1(&input), 150);
    }

    #[test]
    fn test_part1() {
        let input = parse_input(&raw_input());
        assert_eq!(part1(&input), 1451208);
    }
}
