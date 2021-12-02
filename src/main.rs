use std::time::Instant;

mod day01;

fn measure_and_print(day: i32, part: i32, func: &dyn Fn() -> i32) {
    let now = Instant::now();
    let result = func();
    let elapsed = now.elapsed();
    println!("Day {}, Part {}: {} ({:?}ms)", day, part, result, elapsed);
}

fn main() {
    let day01_input = day01::parse_input(&day01::raw_input());

    measure_and_print(1, 1, &|| day01::part1(&day01_input));
    measure_and_print(1, 2, &|| day01::part2(&day01_input));
}
