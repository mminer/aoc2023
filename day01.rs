use std::io::{self, Read};

const NUMBERS: [(usize, &str); 9] = [
    (1, "one"),
    (2, "two"),
    (3, "three"),
    (4, "four"),
    (5, "five"),
    (6, "six"),
    (7, "seven"),
    (8, "eight"),
    (9, "nine"),
];

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let first_digit = find_digit(line.chars());
            let last_digit = find_digit(line.chars().rev());

            format!("{first_digit}{last_digit}")
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let first_number = find_first_number(line);
            let last_number = find_last_number(line);

            format!("{first_number}{last_number}")
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

fn find_digit(mut chars: impl Iterator<Item = char>) -> usize {
    chars
        .find(|c| c.is_numeric())
        .and_then(|c| c.to_digit(10))
        .unwrap() as usize
}

fn find_first_number(line: &str) -> usize {
    let mut result = 0;
    let mut first_index = None;

    for (number, word) in NUMBERS.iter() {
        if let Some(index) = line.find(word) {
            if first_index.is_none() || index < first_index.unwrap() {
                result = *number;
                first_index = Some(index);
            }
        }
    }

    if let Some(index) = line.find(|c: char| c.is_numeric()) {
        if first_index.is_none() || index < first_index.unwrap() {
            result = get_digit(line, index);
        }
    }

    result
}

fn find_last_number(line: &str) -> usize {
    let mut result = 0;
    let mut last_index = None;

    for (number, word) in NUMBERS.iter() {
        if let Some(index) = line.rfind(word) {
            if last_index.is_none() || index > last_index.unwrap() {
                result = *number;
                last_index = Some(index);
            }
        }
    }

    if let Some(index) = line.rfind(|c: char| c.is_numeric()) {
        if last_index.is_none() || index > last_index.unwrap() {
            result = get_digit(line, index);
        }
    }

    result
}

fn get_digit(line: &str, index: usize) -> usize {
    line.chars()
        .nth(index)
        .and_then(|c| c.to_digit(10))
        .unwrap() as usize
}

#[test]
fn sample() {
    let input = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    assert_eq!(part_1(input), 142);

    let input = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    assert_eq!(part_2(input), 281);
}
