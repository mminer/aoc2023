use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    parse_sequence(input).into_iter().map(calculate_hash).sum()
}

fn part_2(input: &str) -> usize {
    let mut boxes: HashMap<usize, Vec<(&str, usize)>> = HashMap::new();

    parse_sequence(input).iter().for_each(|step| {
        let (label, focal_length) = parse_step(step);
        let box_number = calculate_hash(label);
        let lenses = boxes.entry(box_number).or_default();

        if let Some(focal_length) = focal_length {
            if let Some(i) = lenses.iter().position(|lens| lens.0 == label) {
                lenses[i] = (label, focal_length);
            } else {
                lenses.push((label, focal_length));
            }
        } else {
            lenses.retain(|lens| lens.0 != label);
        }
    });

    calculate_focusing_power(&boxes)
}

fn calculate_focusing_power(boxes: &HashMap<usize, Vec<(&str, usize)>>) -> usize {
    let mut result = 0;

    for (box_number, lenses) in boxes.iter() {
        for (i, (_, focal_length)) in lenses.iter().enumerate() {
            let slot_number = i + 1;
            let lens_focusing_power = (1 + box_number) * slot_number * focal_length;
            result += lens_focusing_power;
        }
    }

    result
}

fn calculate_hash(text: &str) -> usize {
    text.chars().fold(0, |mut hash, c| {
        hash += c as usize;
        hash *= 17;
        hash %= 256;
        hash
    })
}

fn parse_sequence(input: &str) -> Vec<&str> {
    input.trim().split(',').collect()
}

fn parse_step(step: &str) -> (&str, Option<usize>) {
    if let Some((label, focal_length_str)) = step.split_once('=') {
        let focal_length = focal_length_str.parse().unwrap();
        (label, Some(focal_length))
    } else {
        let label = step.trim_end_matches('-');
        (label, None)
    }
}

#[test]
fn sample() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(part_1(input), 1320);
    assert_eq!(part_2(input), 145);
}
