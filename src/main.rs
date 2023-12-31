use std::{fs::{self}, env};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

type Solution = fn(Vec<String>) -> (String, String);

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let fname = format!("day{}.txt", day);
    let input = load_input(fname);
    let solution: Solution = match day.as_str() {
        "1" => day1::solution,
        "2" => day2::solution,
        "3" => day3::solution,
        "4" => day4::solution,
        "5" => day5::solution,
        "6" => day6::solution,
        "7" => day7::solution,
        "8" => day8::solution,
        "9" => day9::solution,
        "10" => day10::solution,
        _ => |_| (String::new(), String::new())
    };
    let (part1, part2) = solution(input);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn load_input(fname: String) -> Vec<String> {
    let path = format!("input/{}", fname);
    println!("{path}");
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|e| e.to_owned())
        .collect()
}
