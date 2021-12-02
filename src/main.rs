use std::time::Instant;

mod day01;
mod day02;

fn measure_and_print(day: i32, part: i32, func: &dyn Fn() -> i32) {
    let now = Instant::now();
    let result = func();
    let elapsed = now.elapsed();
    println!(
        "Day {day:02}, Part {part}: {result} ({elapsed:?})",
        day = day,
        part = part,
        result = result,
        elapsed = elapsed
    );
}

fn main() {
    let day01_input = day01::parse_input(&day01::raw_input());
    measure_and_print(1, 1, &|| day01::part1(&day01_input));
    measure_and_print(1, 2, &|| day01::part2(&day01_input));

    let day02_input = day02::parse_input(&day02::raw_input());
    measure_and_print(2, 1, &|| day02::part1(&day02_input));
}
