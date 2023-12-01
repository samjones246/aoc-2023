use std::{fs::{self}, env};

mod day1;

type Solution = fn(Vec<String>) -> String;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let fname = format!("day{}.txt", day);
    let input = load_input(fname);
    let solution: Solution = match day.as_str() {
        "1" => day1::solution,
        _ => |_| String::new()
    };
    let result = solution(input);
    println!("{result}")
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
