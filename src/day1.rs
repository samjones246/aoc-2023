
use regex::Regex;

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
    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|\d").unwrap();
    let re_rev = Regex::new(r"eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d").unwrap();
    let mut result: u32 = 0;
    for line in input {
        let line_rev: String = line.chars().rev().collect();

        let first = re.find(line).unwrap().as_str();
        let first = get_digit(first);

        let last = re_rev.find(&line_rev).unwrap().as_str();
        let last: String = last.chars().rev().collect();
        let last = get_digit(last.as_str());

        let concat = format!("{}{}", first, last);
        let val = concat.parse::<u32>().unwrap();
        result += val;
    }
    return result.to_string();
}

fn get_digit(digit_or_word: &str) -> String {
    let digit_words = 
        vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let result = if let Ok(_) = digit_or_word.parse::<u32>() {
        String::from(digit_or_word)
    } else {
        let index = digit_words
            .iter()
            .position(|w| w == &digit_or_word)
            .unwrap();
        (index + 1).to_string()
    };
    return result;
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

    #[test]
    fn test_part2_basic() {
        let input = vec![
            String::from("two1nine"),
            String::from("eightwothree"),
            String::from("abcone2threexyz"),
            String::from("xtwone3four"),
            String::from("4nineeightseven2"),
            String::from("zoneight234"),
            String::from("7pqrstsixteen")
        ];
        let result = part2(&input);
        assert_eq!(result, "281");
    }

    #[test]
    fn test_part2_all() {
        let input = vec![
            String::from("1nine"),
            String::from("2eight"),
            String::from("3seven"),
            String::from("4six"),
            String::from("5five"),
            String::from("6four"),
            String::from("7three"),
            String::from("8two"),
            String::from("9one"),
        ];
        let result = part2(&input);
        assert_eq!(result, "495");
    }

    #[test]
    fn test_part2_overlap() {
        let input = vec![
            String::from("threeight"),
        ];
        let result = part2(&input);
        assert_eq!(result, "38");
    }
}