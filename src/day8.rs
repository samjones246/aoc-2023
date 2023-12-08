use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_NODE: Regex = Regex::new(r"(\w*) = \((\w*), (\w*)\)").unwrap();
}

pub fn solution(input: Vec<String>) -> (String, String) {
    return (part1(&input), part2(&input));
}

fn part1(input: &Vec<String>) -> String {
    let path = input[0].chars().map(|c| Direction::from(c)).collect_vec();
    let mut nodes: HashMap<&str, Node> = HashMap::new();

    let haystack = &input[2..].join("\n");

    for (_, [name, left, right]) in RE_NODE.captures_iter(&haystack).map(|c| c.extract()) {
        let node = Node { name, left, right };
        nodes.insert(name, node);
    }

    let result = navigate(path, nodes, "AAA", "ZZZ");

    return result.to_string();
}

fn part2(input: &Vec<String>) -> String {
    return String::new();
}

struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

#[derive(PartialEq, Eq)]
enum Direction {
    LEFT,
    RIGHT,
}

impl Direction {
    fn from(c: char) -> Direction {
        match c {
            'L' => Direction::LEFT,
            'R' => Direction::RIGHT,
            _ => panic!("Not a direction")
        }
    }
}

fn navigate(path: Vec<Direction>, nodes: HashMap<&str, Node>, start: &str, end: &str) -> i32 {
    let mut step = 0;
    let mut node = &nodes[start];

    while node.name != end {
        let direction = &path[step % path.len()];
        step += 1;
        let target = if *direction == Direction::LEFT { node.left } else { node.right };
        node = &nodes[target];
    };

    return step as i32;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_basic() {
        let input = vec![
            String::from("RL"),
            String::from(""),
            String::from("AAA = (BBB, CCC)"),
            String::from("BBB = (DDD, EEE)"),
            String::from("CCC = (ZZZ, GGG)"),
            String::from("DDD = (DDD, DDD)"),
            String::from("EEE = (EEE, EEE)"),
            String::from("GGG = (GGG, GGG)"),
            String::from("ZZZ = (ZZZ, ZZZ)"),
        ];

        let result = part1(&input);

        assert_eq!(result, "2");
    }

    #[test]
    fn test_part1_repeat() {
        let input = vec![
            String::from("LLR"),
            String::from(""),
            String::from("AAA = (BBB, BBB)"),
            String::from("BBB = (AAA, ZZZ)"),
            String::from("ZZZ = (ZZZ, ZZZ)"),
        ];

        let result = part1(&input);

        assert_eq!(result, "6");
    }
}