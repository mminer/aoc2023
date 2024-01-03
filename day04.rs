use std::io::{self, Read};

type Card = (Vec<usize>, Vec<usize>); // winning numbers, your numbers

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    parse_cards(input).iter().map(calculate_card_points).sum()
}

fn part_2(input: &str) -> usize {
    let cards = parse_cards(input);
    let mut card_counts = vec![1; cards.len()];

    for (i, (winning_numbers, your_numbers)) in cards.iter().enumerate() {
        let number_of_matches = your_numbers
            .iter()
            .filter(|number| winning_numbers.contains(number))
            .count();

        for offset in 1..=number_of_matches {
            card_counts[i + offset] += card_counts[i];
        }
    }

    card_counts.iter().sum()
}

fn calculate_card_points(card: &Card) -> usize {
    let mut result = 0;
    let (winning_numbers, your_numbers) = card;

    your_numbers
        .iter()
        .filter(|number| winning_numbers.contains(number))
        .for_each(|_| {
            result = if result == 0 { 1 } else { result * 2 };
        });

    result
}

fn parse_cards(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split(": ").last().unwrap().split_once(" | ").unwrap();
            let winning_numbers = parse_numbers(left);
            let your_numbers = parse_numbers(right);
            (winning_numbers, your_numbers)
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
    let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    assert_eq!(part_1(input), 13);
    assert_eq!(part_2(input), 30);
}
