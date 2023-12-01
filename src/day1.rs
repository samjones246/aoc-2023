pub fn solution(input: Vec<String>) -> String {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        let input = vec![
            String::from("1abc2"),
            String::from("pqr3stu8vwx"),
            String::from("a1b2c3d4e5f"),
            String::from("treb7uchet")
        ];
        let result = solution(input);
        assert_eq!(result, "142");
    }
}