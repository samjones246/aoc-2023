use std::{collections::{HashMap, HashSet}, cmp::Ordering};
use itertools::{Itertools, repeat_n};

pub fn solution(input: Vec<String>) -> (String, String) {
    return (part1(&input), part2(&input));
}

fn part1(input: &Vec<String>) -> String {
    let mut result = 0;
    let mut hands: Vec<Hand> = input.iter().map(|l| Hand::from(l)).collect();
    hands.sort();
    for (i, hand) in hands.iter().enumerate() {
        result += hand.bid * (i+1) as u32;
    }
    return result.to_string();
}

fn part2(input: &Vec<String>) -> String {
    let mut result = 0;
    let mut hands: Vec<Hand> = input.iter().map(|l| Hand::from(l)).collect();
    hands.sort();
    for (i, hand) in hands.iter().enumerate() {
        result += hand.bid * (i+1) as u32;
    }
    return result.to_string();
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
enum Card {
    Jack,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from(c: char) -> Card {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Not a card")
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

impl HandType {
    fn of(cards: &Vec<Card>) -> HandType {
        let mut map = HashMap::<&Card, i32>::new();
        for c in cards {
            map.insert(&c, map.get(&c).unwrap_or(&0) + 1);
        };
        if map.values().any(|v| *v == 5) { return HandType::FiveOfAKind; }
        if map.values().any(|v| *v == 4) { return HandType::FourOfAKind; }
        let pairs = map.values().filter(|v| **v == 2).count();
        if pairs == 2 {
            return HandType::TwoPair;
        };
        if map.values().any(|v| *v == 3) {
            if pairs == 1 {
                return HandType::FullHouse;
            }
            return HandType::ThreeOfAKind;
        };
        if pairs == 1 {
            return HandType::OnePair;
        };
        return HandType::HighCard;
    }
}

struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: u32,
}

impl Hand {
    fn from(line: &String) -> Hand {
        let parts: Vec<&str> = line.split(" ").collect();
        let cards: Vec<Card> = parts[0].chars().map(|c| Card::from(c)).collect();
        let without_jacks: Vec<Card> = cards.iter().filter_map(|c| if *c != Card::Jack { Some(c.to_owned()) } else { None }).collect();
        let hand_type: HandType = if without_jacks.len() == 5 {
            HandType::of(&cards)
        } else if without_jacks.len() == 0 {
            HandType::FiveOfAKind
        } else {
            let distinct: HashSet<Card> = HashSet::from_iter(without_jacks.clone());
            let options = repeat_n(distinct.iter(), 5 - without_jacks.len()).multi_cartesian_product()
                .map(|p| {
                    let mut v: Vec<Card> = Vec::new();
                    v.extend(without_jacks.clone());
                    v.extend(p);
                    v
                })
                .collect_vec();
            let hand_types = options.iter().map(|cs| HandType::of(&cs));
            let result = hand_types.clone().max().unwrap();
            result
        };
        let bid: u32 = parts[1].parse().unwrap();
        Hand { cards, hand_type, bid }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
                if c1 == c2 {
                    continue;
                }
                return c1.cmp(c2);
            };
            return Ordering::Equal;
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
       Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_basic() {
        let input = vec![
            String::from("32T3K 765"),
            String::from("T55J5 684"),
            String::from("KK677 28"),
            String::from("KTJJT 220"),
            String::from("QQQJA 483"),
        ];

        let result = part1(&input);

        assert_eq!(result, "6440");
    }

    #[test]
    fn test_part2_basic() {
        let input = vec![
            String::from("32T3K 765"),
            String::from("T55J5 684"),
            String::from("KK677 28"),
            String::from("KTJJT 220"),
            String::from("QQQJA 483"),
        ];

        let result = part2(&input);

        assert_eq!(result, "5905");
    }
}