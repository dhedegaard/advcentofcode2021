use std::time::Instant;

mod day01;

fn main() {
    let day01_input = day01::parse_input(&day01::raw_input());

    let now = Instant::now();
    let result = day01::part1(&day01_input);
    println!("Day 01 - part 1: {} - took: {:?}", result, now.elapsed());
    let now = Instant::now();
    let result = day01::part2(&day01_input);
    println!("Day 01 - part 2: {} - took: {:?}", result, now.elapsed());
}
