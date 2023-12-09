use itertools::Itertools;

pub fn solution(input: Vec<String>) -> (String, String) {
    return (part1(&input), part2(&input));
}

fn part1(input: &Vec<String>) -> String {
    let mut result: i64 = 0;
    for line in input {
        let sequence = line.split(" ").map(|e| e.parse::<i64>().unwrap()).collect_vec();
        let next = extrapolate(sequence);
        result += next;
    }
    return result.to_string();
}

fn part2(input: &Vec<String>) -> String {
    let mut result: i64 = 0;
    for line in input {
        let sequence = line.split(" ").map(|e| e.parse::<i64>().unwrap()).collect_vec();
        let next = extrapolate_back(sequence);
        result += next;
    }
    return result.to_string();
}

fn extrapolate(sequence: Vec<i64>) -> i64 {
    if sequence.iter().all(|e| *e == 0) {
        return 0;
    }
    let mut differences = Vec::new();
    for i in 1..sequence.len() {
        differences.push(
            sequence[i] - sequence[i-1]
        );
    }
    return sequence.last().unwrap() + extrapolate(differences);
}

fn extrapolate_back(sequence: Vec<i64>) -> i64 {
    if sequence.iter().all(|e| *e == 0) {
        return 0;
    }
    let mut differences = Vec::new();
    for i in 1..sequence.len() {
        differences.push(
            sequence[i] - sequence[i-1]
        );
    }
    return sequence.first().unwrap() - extrapolate_back(differences);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_basic() {
        let input = vec![
            String::from("0 3 6 9 12 15"),
            String::from("1 3 6 10 15 21"),
            String::from("10 13 16 21 30 45"),
        ];

        let result = part1(&input);

        assert_eq!(result, "114");
    }

    #[test]
    fn test_part2_basic() {
        let input = vec![
            String::from("0 3 6 9 12 15"),
            String::from("1 3 6 10 15 21"),
            String::from("10 13 16 21 30 45"),
        ];

        let result = part2(&input);

        assert_eq!(result, "2");
    }
}