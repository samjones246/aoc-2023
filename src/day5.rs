use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{min, max};

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
    let result = locations.iter().min().unwrap();

    return result.to_string();
}

fn part2(input: &Vec<String>) -> String {
    let seed_ranges: Vec<u64> = input[0][7..].split(" ").map(|e| e.parse::<u64>().unwrap()).collect();
    let mut seed_ranges: Vec<(u64, u64)> = ((0..seed_ranges.len()).filter(|x| x % 2 == 0))
        .map(|i| (seed_ranges[i], seed_ranges[i + 1])).collect();

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
            rmap.ranges.sort_by_key(|r| r.start);
            maps.push(rmap);
            rmap = ResourceMapping { ranges: Vec::new() };
            continue 
        };
        let values: Vec<u64> = line.split(" ").map(|e| e.parse::<u64>().unwrap()).collect();
        rmap.ranges.push(
            MappingRange { start: values[1], end: values[1] + values[2], dest: values[0] }
        )
    }
    rmap.ranges.sort_by_key(|r| r.start);
    maps.push(rmap);

    let mut results = Vec::new();
    for (start, length) in &seed_ranges {
        results.push(
            apply_maps_range(&maps, *start, *length)
        );
    }

    return results.iter().min().unwrap().to_string();
}

#[derive(Clone)]
struct ResourceMapping {
    ranges: Vec<MappingRange>
}

#[derive(Clone, Copy)]
struct MappingRange {
    start: u64,
    end: u64,
    dest: u64,
}

fn apply_maps(maps: &Vec<ResourceMapping>, value: u64) -> u64 {
    let mut result = value;
    for map in maps {
        for range in &map.ranges {
            if result >= range.start && result < range.end {
                let depth = result - range.start;
                result = range.dest + depth;
                break;
            }
        }
    }
    return result;
}

fn apply_maps_range(maps: &Vec<ResourceMapping>, range_start: u64, range_length: u64) -> u64 {
    let mut new_ranges = Vec::new();
    let mut start = range_start;
    let mut length = range_length;
    for (i, map) in maps.iter().enumerate() {
        let end = start + length;
        for range in &map.ranges {
            if start < range.start { // Input is before all ranges
                if end >= range.start {
                    let other_start = range.start;
                    let other_length = end - range.start;
                    new_ranges.push((other_start, other_length, i));
                    length = range.start - start;
                }
                break;
            } else if start < range.end { // Input is in this range
                if end >= range.end {
                    let other_start = range.end;
                    let other_length = end - range.end;
                    new_ranges.push((other_start, other_length, i));
                    length = range.end - start;
                }
                start = range.dest + (start - range.start);
                break;
            }
        }
    }
    let other_result = new_ranges.iter()
        .map(|(other_start, other_length, depth)| {
            let sub_maps = &maps[*depth..].to_vec();
            apply_maps_range(sub_maps, *other_start, *other_length)
        }).min().unwrap_or(start + 1);
    let result = min(start, other_result);
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

    #[test]
    fn test_part2_basic() {
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

        let (_, result) = solution(input);

        assert_eq!(result, "46");
    }
}