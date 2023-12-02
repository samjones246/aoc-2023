use std::{collections::HashMap, cmp::max};

pub fn solution(input: Vec<String>) -> (String, String) {
    let mut games = Vec::new();
    for line in &input {
        let game = Game::from_input(line);
        games.push(game);
    }
    return (part1(&games), part2(&games));
}

fn part1(games: &Vec<Game>) -> String {
    let mut result: u32 = 0;
    for game in games {
        let mut possible = true;
        for draw in &game.draws {
            let red = draw.get(&Colour::RED).unwrap_or(&0);
            let green = draw.get(&Colour::GREEN).unwrap_or(&0);
            let blue = draw.get(&Colour::BLUE).unwrap_or(&0);
            if red > &12 || green > &13 || blue > &14 {
                possible = false;
                break;
            }
        }
        if possible {
            result += game.id;
        }
    }
    return result.to_string();
}
 
fn part2(games: &Vec<Game>) -> String {
    let mut result: u32 = 0;
    for game in games {
        let mut min_red: u32 = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for draw in &game.draws {
            let red = *draw.get(&Colour::RED).unwrap_or(&0);
            let green = *draw.get(&Colour::GREEN).unwrap_or(&0);
            let blue = *draw.get(&Colour::BLUE).unwrap_or(&0);
            min_red = max(min_red, red);
            min_green = max(min_green, green);
            min_blue = max(min_blue, blue);
        }
        let power = min_red * min_green * min_blue;
        result += power;
    }
    return result.to_string();
}

struct Game {
    id: u32,
    draws: Vec<HashMap<Colour, u32>>
}

impl Game {
    fn from_input(line: &String) -> Game {
        let parts: Vec<&str> = line.split(":").collect();
        let (id_str, tail) = parts.split_first().unwrap();
        let id = id_str.split(" ").last().unwrap().parse::<u32>().unwrap();

        let draw_strs: Vec<&str> = tail[0].split(";").collect();
        let mut draws = Vec::new();
        for draw_str in draw_strs {
            let groups_str: Vec<&str> = draw_str.split(",").map(|s| s.trim()).collect();
            let mut groups: HashMap<Colour, u32> = HashMap::new();
            for group in groups_str {
                let temp: Vec<&str>  = group.split(" ").collect();
                let num = temp[0].parse::<u32>().unwrap();
                let colour = Colour::from_str(temp[1]);
                groups.insert(colour, num);
            }
            draws.push(groups);
        };
        Game { id, draws }
    }
}

#[derive(PartialEq, Eq, Hash)]
enum Colour {
    RED,
    GREEN,
    BLUE
}

impl Colour {
    fn from_str(str: &str) -> Colour {
        match str {
            "red" => Colour::RED,
            "green" => Colour::GREEN,
            "blue" => Colour::BLUE,
            _ => panic!("not a colour")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1_basic() {
        let input = vec![
            String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            String::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            String::from("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
        ];
        let (result, _) = solution(input);
        assert_eq!(result, "8")
    }

    #[test]
    fn test_part2_basic() {
        let input = vec![
            String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            String::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"),
            String::from("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
        ];
        let (_, result) = solution(input);
        assert_eq!(result, "2286")
    }
}