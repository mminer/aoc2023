use std::io::{self, Read};
use std::iter::Peekable;
use std::ops::Range;
use std::str::Lines;
use std::thread;

type Map = Vec<(Range<usize>, usize)>; // source range, destination range start

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let mut lines = input.lines().peekable();

    let seeds = parse_numbers(lines.next().unwrap().split(": ").last().unwrap());
    lines.next();

    let mut maps = Vec::new();

    while lines.peek().is_some() {
        let map = parse_map(&mut lines);
        maps.push(map);
    }

    seeds
        .into_iter()
        .map(|seed| {
            maps.iter()
                .fold(seed, |source, map| get_destination(map, source))
        })
        .min()
        .unwrap()
}

fn part_2(input: &str) -> usize {
    let mut lines = input.lines().peekable();

    let seed_ranges: Vec<_> = parse_numbers(lines.next().unwrap().split(": ").last().unwrap())
        .chunks(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1] - 1))
        .collect();

    lines.next();

    let mut maps = Vec::new();

    while lines.peek().is_some() {
        let map = parse_map(&mut lines);
        maps.push(map);
    }

    // This is brute forcing the solution. Threads make it faster, but even so it's slow.
    // I need to rethink the approach.
    let handles: Vec<_> = seed_ranges
        .into_iter()
        .map(|range| {
            let maps = maps.clone();

            thread::spawn(move || {
                range
                    .map(|seed| {
                        maps.iter()
                            .fold(seed, |source, map| get_destination(map, source))
                    })
                    .min()
                    .unwrap()
            })
        })
        .collect();

    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .min()
        .unwrap()
}

fn get_destination(map: &Map, source: usize) -> usize {
    map.iter()
        .find(|(source_range, _)| source_range.contains(&source))
        .map(|(source_range, destination_range_start)| {
            let range_index = source - source_range.start;
            destination_range_start + range_index
        })
        .unwrap_or(source)
}

fn parse_map(lines: &mut Peekable<Lines>) -> Map {
    // Ignore header.
    lines.next();

    lines
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let numbers = parse_numbers(line);
            let destination_range_start = numbers[0];
            let source_range_start = numbers[1];
            let range_length = numbers[2];

            let source_range_end = source_range_start + range_length;
            let source_range = source_range_start..source_range_end;

            (source_range, destination_range_start)
        })
        .collect()
}

fn parse_numbers(text: &str) -> Vec<usize> {
    text.split_whitespace()
        .map(|number_str| number_str.parse().unwrap())
        .collect()
}

#[test]
fn sample() {
    let input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    assert_eq!(part_1(input), 35);
    assert_eq!(part_2(input), 46);
}
