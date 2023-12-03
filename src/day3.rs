
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_NUM: Regex = Regex::new(r"\d+").unwrap();
    static ref RE_SYM: Regex = Regex::new(r"[^\d\.]").unwrap();
    static ref RE_GEAR: Regex = Regex::new(r"\*").unwrap();
}

pub fn solution(input: Vec<String>) -> (String, String) {
    return (part1(&input), part2(&input));
}

fn part1(input: &Vec<String>) -> String {
    let mut result: u32 = 0;

    let rows = input.len();
    let cols = input[0].len();

    let mut symbols = vec![vec![false; cols]; rows];

    for (line_num, line) in input.iter().enumerate() {
        RE_SYM.find_iter(&line).for_each(|m| {
            symbols[line_num][m.start()] = true;
        });
    };

    for (line_num, line) in input.iter().enumerate() {
        RE_NUM.find_iter(line).for_each(|m| {
            let mut valid = false;
            let mstart = m.start();
            let mend = m.end() - 1;
            let start_row = if line_num > 0 { line_num - 1 } else { 0 };
            let end_row = if line_num < symbols.len() - 1 { line_num + 1 } else { line_num };
            'outer: for i in start_row..=end_row {
                let start_col = if mstart > 0 { mstart - 1 } else { 0 };
                let end_col = if mend < symbols[i].len() - 1 { mend + 1 } else { mend };
                for j in start_col..=end_col {
                    if symbols[i][j] {
                        valid = true;
                        break 'outer;
                    }
                }
            }
            if valid {
                result += m.as_str().parse::<u32>().unwrap();
            }
        });
    };

    return result.to_string();
}

fn part2(input: &Vec<String>) -> String {
    let mut result: u32 = 0;

    let rows = input.len();
    let cols = input[0].len();

    let mut adjacent_numbers = vec![vec![Vec::new(); cols]; rows];

    for (line_num, line) in input.iter().enumerate() {
        RE_NUM.find_iter(line).for_each(|m| {
            let value: u32 = m.as_str().parse().unwrap();
            let mstart = m.start();
            let mend = m.end() - 1;
            let start_row = if line_num > 0 { line_num - 1 } else { 0 };
            let end_row = if line_num < adjacent_numbers.len() - 1 { line_num + 1 } else { line_num };
            for i in start_row..=end_row {
                let start_col = if mstart > 0 { mstart - 1 } else { 0 };
                let end_col = if mend < adjacent_numbers[i].len() - 1 { mend + 1 } else { mend };
                for j in start_col..=end_col {
                    adjacent_numbers[i][j].push(value)
                }
            }
        });
    };

    for (line_num, line) in input.iter().enumerate() {
        RE_GEAR.find_iter(&line).for_each(|m| {
            let nums = &adjacent_numbers[line_num][m.start()];
            if nums.len() == 2 {
                let ratio = nums[0] * nums[1];
                result += ratio;
            }
        });
    };

    return result.to_string();
}

#[cfg(test)]
mod test {
    use super::solution;

    #[test]
    fn test_part1_basic() {
        let input = vec![
            String::from("467..114.."),
            String::from("...*......"),
            String::from("..35..633."),
            String::from("......#..."),
            String::from("617*......"),
            String::from(".....+.58."),
            String::from("..592....."),
            String::from("......755."),
            String::from("...$.*...."),
            String::from(".664.598.."),
        ];

        let (result, _) = solution(input);
        assert_eq!(result, "4361");
    }

    #[test]
    fn test_part2_basic() {
        let input = vec![
            String::from("467..114.."),
            String::from("...*......"),
            String::from("..35..633."),
            String::from("......#..."),
            String::from("617*......"),
            String::from(".....+.58."),
            String::from("..592....."),
            String::from("......755."),
            String::from("...$.*...."),
            String::from(".664.598.."),
        ];

        let (_, result) = solution(input);
        assert_eq!(result, "467835");
    }
}