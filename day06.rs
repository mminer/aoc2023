use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let mut lines = input.lines();
    let race_durations = parse_numbers(lines.next().unwrap());
    let records = parse_numbers(lines.next().unwrap());

    race_durations
        .into_iter()
        .zip(records)
        .map(|(race_duration, record)| count_ways_to_beat_record(race_duration, record))
        .product()
}

fn part_2(input: &str) -> usize {
    let mut lines = input.lines();
    let race_duration = parse_number_removing_spaces(lines.next().unwrap());
    let record = parse_number_removing_spaces(lines.next().unwrap());
    count_ways_to_beat_record(race_duration, record)
}

fn count_ways_to_beat_record(race_duration: usize, record: usize) -> usize {
    (1..race_duration)
        .map(|button_duration| (race_duration - button_duration) * button_duration)
        .filter(|&time| time > record)
        .count()
}

fn parse_number_removing_spaces(line: &str) -> usize {
    line.split(':')
        .last()
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap()
}

fn parse_numbers(line: &str) -> Vec<usize> {
    line.split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|number_str| number_str.parse().unwrap())
        .collect()
}

#[test]
fn sample() {
    let input = r"Time:      7  15   30
Distance:  9  40  200";

    assert_eq!(part_1(input), 288);
    assert_eq!(part_2(input), 71503);
}
