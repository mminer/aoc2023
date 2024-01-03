use std::io::{self, Read};

const DAMAGED: char = '#';
const OPERATIONAL: char = '.';
const UNKNOWN: char = '?';

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    input.lines().map(calculate_possible_arrangements).sum()
}

fn part_2(input: &str) -> usize {
    // TODO
    0
}

fn calculate_possible_arrangements(line: &str) -> usize {
    let (conditions, damaged_group_sizes) = parse_record(line);
    let known_damaged_count = conditions.iter().filter(|&c| *c == DAMAGED).count();
    let unknown_count = conditions.iter().filter(|&c| *c == UNKNOWN).count();
    let total_spring_count: usize = damaged_group_sizes.iter().sum();

    get_unknowns_permutations(unknown_count, total_spring_count - known_damaged_count)
        .iter()
        .map(|unknowns_permutation| create_arrangement(&conditions, unknowns_permutation))
        .filter(|arrangement| is_arrangement_valid(arrangement, &damaged_group_sizes))
        .count()
}

fn count_repeated_values(values: &[char]) -> Vec<(char, usize)> {
    values
        .iter()
        .fold(vec![], |mut counts, value| {
            if let Some((previous_value, repeat_count)) = counts.last_mut() {
                if value == previous_value {
                    *repeat_count += 1;
                } else {
                    counts.push((*value, 1));
                }
            } else {
                counts.push((*value, 1));
            }

            counts
        })
        .into_iter()
        .map(|(value, repeat_count)| (value, repeat_count))
        .collect()
}

fn create_arrangement(conditions: &[char], unknowns_permutation: &[char]) -> Vec<char> {
    let mut unknowns_permutation = unknowns_permutation.iter();

    conditions
        .iter()
        .map(|&c| match c {
            UNKNOWN => *unknowns_permutation.next().unwrap(),
            c => c,
        })
        .collect()
}

fn get_unknowns_permutations(
    unknown_count: usize,
    required_damaged_count: usize,
) -> Vec<Vec<char>> {
    if required_damaged_count > unknown_count {
        return vec![];
    }

    if unknown_count == 0 {
        return vec![vec![]];
    }

    if required_damaged_count == 0 {
        return vec![vec![OPERATIONAL; unknown_count]];
    }

    let mut result = Vec::new();

    for mut damaged in get_unknowns_permutations(unknown_count - 1, required_damaged_count - 1) {
        damaged.push(DAMAGED);
        result.push(damaged);
    }

    for mut operational in get_unknowns_permutations(unknown_count - 1, required_damaged_count) {
        operational.push(OPERATIONAL);
        result.push(operational);
    }

    result
}

fn is_arrangement_valid(arrangement: &[char], damaged_group_sizes: &[usize]) -> bool {
    let arrangement_damaged_group_sizes: Vec<_> = count_repeated_values(arrangement)
        .into_iter()
        .filter(|(value, _)| *value == DAMAGED)
        .map(|(_, repeat_count)| repeat_count)
        .collect();

    arrangement_damaged_group_sizes == damaged_group_sizes
}

fn parse_record(line: &str) -> (Vec<char>, Vec<usize>) {
    let (conditions_str, damaged_group_sizes_str) = line.split_once(' ').unwrap();
    let conditions = conditions_str.chars().collect();

    let damaged_group_sizes = damaged_group_sizes_str
        .split(',')
        .map(|number_str| number_str.parse().unwrap())
        .collect();

    (conditions, damaged_group_sizes)
}

#[test]
fn sample() {
    [
        ("???.### 1,1,3", 1),
        (".??..??...?##. 1,1,3", 4),
        ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
        ("????.#...#... 4,1,1", 1),
        ("????.######..#####. 1,6,5", 4),
        ("?###???????? 3,2,1", 10),
    ]
    .iter()
    .for_each(|(line, expected)| assert_eq!(calculate_possible_arrangements(line), *expected));

    let input = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    assert_eq!(part_1(input), 21);
    assert_eq!(part_2(input), 0);
}
