pub fn solution(input: Vec<String>) -> (String, String) {
    return (part1(&input), part2(&input))
}

fn part1(input: &Vec<String>) -> String {
    let mut result: u32 = 0;
    for line in input {
        let digits: Vec<char> = line
            .chars()
            .filter(|c| c.is_digit(10))
            .collect();
        let concat = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
        result += concat.parse::<u32>().unwrap();
    }
    return result.to_string()
}

fn part2(input: &Vec<String>) -> String {
    return String::new();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_basic() {
        let input = vec![
            String::from("1abc2"),
            String::from("pqr3stu8vwx"),
            String::from("a1b2c3d4e5f"),
            String::from("treb7uchet")
        ];
        let result = part1(&input);
        assert_eq!(result, "142");
    }
}