pub fn solution(input: Vec<String>) -> (String, String) {
    return (part1(&input), part2(&input));
}

fn part1(input: &Vec<String>) -> String {
    let extract = |line: &String| line
        .split_ascii_whitespace()
        .skip(1)
        .map(|e| e.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let times = extract(&input[0]);
    let distances = extract(&input[1]);
    let result = times.iter().zip(distances.iter())
        .map(|(time, distance)| solve(*time, *distance))
        .reduce(|a, b| a * b)
        .unwrap();
    return result.to_string()
}

fn part2(input: &Vec<String>) -> String {
    return String::new();
}

fn solve(time: i32, distance: i32) -> i32 {
    // a = -1, b = time, c = -distance
    let disc = (time * time) - (4 * distance);
    let r_disc = (disc as f32).sqrt();
    let roots: Vec<f32> = vec![r_disc, -r_disc as f32].iter().map(
        |d| ((-time as f32 + d) / -2.0)
    )
    .collect();
    let result = roots[1].ceil() - roots[0].floor() - 1.0;
    return result as i32;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1_basic() {
        let input = vec![
            String::from("Time:      7  15   30"),
            String::from("Distance:  9  40  200"),
        ];

        let result = part1(&input);

        assert_eq!(result, "288");
    }
}