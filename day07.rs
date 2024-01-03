use std::collections::HashMap;
use std::convert::TryInto;
use std::io::{self, Read};

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from(c: char, jokers_present: bool) -> Card {
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
            'J' => {
                if jokers_present {
                    Card::Joker
                } else {
                    Card::Jack
                }
            }
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!(),
        }
    }
}

type Hand = [Card; 5];

#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let mut hands = parse_hands(input, false);
    hands.sort_by_key(|(hand, _)| (get_hand_type_ignoring_jokers(hand), hand.clone()));
    calculate_total_winnings(hands)
}

fn part_2(input: &str) -> usize {
    let mut hands = parse_hands(input, true);
    hands.sort_by_key(|(hand, _)| (get_hand_type(hand), hand.clone()));
    calculate_total_winnings(hands)
}

fn calculate_total_winnings(hands: Vec<(Hand, usize)>) -> usize {
    hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| {
            let rank = i + 1;
            bid * rank
        })
        .sum()
}

fn get_hand_type(hand: &Hand) -> HandType {
    let hand_type = get_hand_type_ignoring_jokers(hand);
    let joker_count = hand.iter().filter(|&card| card == &Card::Joker).count();

    match (hand_type, joker_count) {
        (HandType::HighCard, 1) => HandType::OnePair,
        (HandType::HighCard, 2) => HandType::ThreeOfAKind,
        (HandType::HighCard, 3) => HandType::FourOfAKind,
        (HandType::HighCard, 4) => HandType::FiveOfAKind,
        (HandType::HighCard, 5) => HandType::FiveOfAKind,

        (HandType::OnePair, 1) => HandType::ThreeOfAKind,
        (HandType::OnePair, 2) => HandType::FourOfAKind,
        (HandType::OnePair, 3) => HandType::FiveOfAKind,

        (HandType::TwoPair, 1) => HandType::FullHouse,

        (HandType::ThreeOfAKind, 1) => HandType::FourOfAKind,
        (HandType::ThreeOfAKind, 2) => HandType::FiveOfAKind,

        (HandType::FourOfAKind, 1) => HandType::FiveOfAKind,

        (hand_type, _) => hand_type,
    }
}

fn get_hand_type_ignoring_jokers(hand: &Hand) -> HandType {
    let card_counts: Vec<_> = hand
        .iter()
        .filter(|&card| card != &Card::Joker)
        .fold(HashMap::new(), |mut map, card| {
            map.entry(card).and_modify(|count| *count += 1).or_insert(1);
            map
        })
        .into_values()
        .collect();

    if card_counts.contains(&5) {
        return HandType::FiveOfAKind;
    }

    if card_counts.contains(&4) {
        return HandType::FourOfAKind;
    }

    if card_counts.contains(&3) && card_counts.contains(&2) {
        return HandType::FullHouse;
    }

    if card_counts.contains(&3) {
        return HandType::ThreeOfAKind;
    }

    if card_counts.iter().filter(|&count| count == &2).count() == 2 {
        return HandType::TwoPair;
    }

    if card_counts.contains(&2) {
        return HandType::OnePair;
    }

    HandType::HighCard
}

fn parse_hands(input: &str, jokers_present: bool) -> Vec<(Hand, usize)> {
    input
        .lines()
        .map(|line| {
            let (hand_str, bid_str) = line.split_once(' ').unwrap();

            let hand = hand_str
                .chars()
                .map(|c| Card::from(c, jokers_present))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            let bid = bid_str.parse().unwrap();
            (hand, bid)
        })
        .collect()
}

#[test]
fn sample() {
    let input = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    assert_eq!(part_1(input), 6440);
    assert_eq!(part_2(input), 5905);
}
