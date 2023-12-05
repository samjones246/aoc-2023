use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_MAP: Regex = Regex::new(r"(\d*) (\d*) (\d*)").unwrap();
}

pub fn solution(input: Vec<String>) -> (String, String) {
    return (part1(&input), part2(&input));
}

fn part1(input: &Vec<String>) -> String {
    let seeds: Vec<u64> = input[0][7..].split(" ").map(|e| e.parse::<u64>().unwrap()).collect();
    let mut maps = Vec::new();
    let mut rmap = ResourceMapping { ranges: Vec::new() };
    let mut skip = false;
    for line in &input[3..] {
        if skip {
            skip = false;
            continue;
        }
        if line.is_empty() { 
            skip = true;
            maps.push(rmap);
            rmap = ResourceMapping { ranges: Vec::new() };
            continue 
        };
        let values: Vec<u64> = line.split(" ").map(|e| e.parse::<u64>().unwrap()).collect();
        rmap.ranges.push(
            MappingRange { start: values[1], end: values[1] + values[2], dest: values[0] }
        )
    }
    maps.push(rmap);
    let locations: Vec<u64> = seeds.iter().map(|s| apply_maps(&maps, *s)).collect();
    // println!("locations: {locations:?}");
    let result = locations.iter().min().unwrap();

    return result.to_string();
}

fn part2(input: &Vec<String>) -> String {
    let mut result = 0;

    return result.to_string();
}

struct ResourceMapping {
    ranges: Vec<MappingRange>
}

struct MappingRange {
    start: u64,
    end: u64,
    dest: u64,
}

fn apply_maps(maps: &Vec<ResourceMapping>, value: u64) -> u64 {
    let mut result = value;
    // println!("input: {value}");
    for map in maps {
        for range in &map.ranges {
            if result >= range.start && result < range.end {
                let depth = result - range.start;
                result = range.dest + depth;
                break;
            }
        }
        // println!("-> {result}");
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1_basic() {
        let input = vec![
            String::from("seeds: 79 14 55 13"),
            String::from(""),
            String::from("seed-to-soil map:"),
            String::from("50 98 2"),
            String::from("52 50 48"),
            String::from(""),
            String::from("soil-to-fertilizer map:"),
            String::from("0 15 37"),
            String::from("37 52 2"),
            String::from("39 0 15"),
            String::from(""),
            String::from("fertilizer-to-water map:"),
            String::from("49 53 8"),
            String::from("0 11 42"),
            String::from("42 0 7"),
            String::from("57 7 4"),
            String::from(""),
            String::from("water-to-light map:"),
            String::from("88 18 7"),
            String::from("18 25 70"),
            String::from(""),
            String::from("light-to-temperature map:"),
            String::from("45 77 23"),
            String::from("81 45 19"),
            String::from("68 64 13"),
            String::from(""),
            String::from("temperature-to-humidity map:"),
            String::from("0 69 1"),
            String::from("1 0 69"),
            String::from(""),
            String::from("humidity-to-location map:"),
            String::from("60 56 37"),
            String::from("56 93 4"),
        ];

        let (result, _) = solution(input);

        assert_eq!(result, "35");
    }
}