pub fn solution(input: Vec<String>) -> (String, String) {
    return (part1(&input), part2(&input));
}

fn part1(input: &Vec<String>) -> String {
    let mut result = 0;
    for line in input {
        let parts: Vec<&str> = line.split(":").last().unwrap().split("|").collect();
        let winning: Vec<&str> = parts[0].trim().split(" ").filter(|e| *e != "").collect();
        let matches: u32 = parts[1].trim().split(" ").filter(|e| *e != "").filter(|e| winning.contains(e)).count().try_into().unwrap();
        if matches > 0 {
            result += 2_u32.pow(matches-1);
        }
    };

    return result.to_string();
}

fn part2(input: &Vec<String>) -> String {
    let mut result = 0;
    let num_unique_cards = input.len();
    let mut card_counts = vec![1_usize; num_unique_cards];
    for (index, line) in input.iter().enumerate() {
        let mult = card_counts[index];
        let parts: Vec<&str> = line.split(":").last().unwrap().split("|").collect();
        let winning: Vec<&str> = parts[0].trim().split(" ").filter(|e| *e != "").collect();
        let matches: usize = parts[1].trim().split(" ").filter(|e| *e != "").filter(|e| winning.contains(e)).count().try_into().unwrap();
        for i in 1..=matches {
            card_counts[index + i] += mult;
        }
    };

    return card_counts.iter().sum::<usize>().to_string();
}

#[cfg(test)]
mod test {
    use super::solution;

    #[test]
    fn test_part1_basic() {
        let input = vec![
            String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            String::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            String::from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            String::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            String::from("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
        ];

        let (result, _) = solution(input);

        assert_eq!(result, "13");
    }

    #[test]
    fn test_part2_basic() {
        let input = vec![
            String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            String::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            String::from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            String::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            String::from("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
        ];

        let (_, result) = solution(input);

        assert_eq!(result, "30");
    }
}