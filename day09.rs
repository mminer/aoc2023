use std::io::{self, Read};
use std::iter;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> isize {
    parse_report(input).iter().map(extrapolate_next_value).sum()
}

fn part_2(input: &str) -> isize {
    parse_report(input).iter().map(extrapolate_prev_value).sum()
}

fn calculate_next_sequence(sequence: &Vec<isize>) -> Vec<isize> {
    (0..(sequence.len() - 1))
        .map(|i| sequence[i + 1] - sequence[i])
        .collect()
}

fn create_sequences(history: &[isize]) -> Vec<Vec<isize>> {
    iter::successors(Some(history.to_owned()), |sequence| {
        if sequence.iter().all(|&value| value == 0) {
            return None;
        }

        Some(calculate_next_sequence(sequence))
    })
    .collect()
}

fn extrapolate_next_value(history: &Vec<isize>) -> isize {
    let mut sequences = create_sequences(history);

    for i in (1..sequences.len()).rev() {
        let extrapolated_value = sequences[i - 1].last().unwrap() + sequences[i].last().unwrap();
        sequences[i - 1].push(extrapolated_value);
    }

    *sequences[0].last().unwrap()
}

fn extrapolate_prev_value(history: &Vec<isize>) -> isize {
    let mut sequences = create_sequences(history);

    for i in (1..sequences.len()).rev() {
        let extrapolated_value = sequences[i - 1].first().unwrap() - sequences[i].first().unwrap();
        sequences[i - 1].insert(0, extrapolated_value);
    }

    *sequences[0].first().unwrap()
}

fn parse_report(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|number_str| number_str.parse().unwrap())
                .collect()
        })
        .collect()
}

#[test]
fn sample() {
    let input = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    assert_eq!(part_1(input), 114);
    assert_eq!(part_2(input), 2);
}
