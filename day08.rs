use std::collections::HashMap;
use std::io::{self, Read};

type Network<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let (instructions, network) = parse_map(input);
    count_steps_for_node(instructions, &network, "AAA", "ZZZ")
}

fn part_2(input: &str) -> usize {
    let (instructions, network) = parse_map(input);

    // Insight: The answer requires finding a path to the end node from each start node, then
    // cycling through these paths simultaneously until all of them reach their end node at the
    // same time. The paths are all different lengths, but each path cycle takes the same number of
    // steps to reach its end node. This means that we can take a shortcut and find the least
    // common multiple of all path steps instead of actually cycling through the paths repeatedly.
    //
    // This assumes that the cycle of directions will be the same for each path cycle, which isn't
    // guaranteed? But it does appear to be true for the puzzle input.
    network
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|node| count_steps_for_node(instructions, &network, node, "Z"))
        .reduce(least_common_multiple)
        .unwrap()
}

fn count_steps_for_node(
    instructions: &str,
    network: &Network,
    start_node: &str,
    end_node_suffix: &str,
) -> usize {
    instructions
        .chars()
        .cycle()
        .scan(start_node, |node, direction| {
            if node.ends_with(end_node_suffix) {
                return None;
            }

            *node = match direction {
                'L' => network[node].0,
                'R' => network[node].1,
                _ => panic!(),
            };

            Some(*node)
        })
        .count()
}

fn greatest_common_divisor(mut a: usize, mut b: usize) -> usize {
    // https://en.wikipedia.org/wiki/Greatest_common_divisor#Euclidean_algorithm
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

fn least_common_multiple(a: usize, b: usize) -> usize {
    // https://en.wikipedia.org/wiki/Least_common_multiple#Calculation
    a * b / greatest_common_divisor(a, b)
}

fn parse_map(input: &str) -> (&str, Network) {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    lines.next();

    let network = lines
        .map(|line| {
            let node = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];
            (node, (left, right))
        })
        .collect();

    (instructions, network)
}

#[test]
fn sample() {
    let input = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    assert_eq!(part_1(input), 2);

    let input = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    assert_eq!(part_1(input), 6);

    let input = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    assert_eq!(part_2(input), 6);
}
