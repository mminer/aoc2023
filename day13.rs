use std::io::{self, Read};
use std::iter;

type Pattern = Vec<Vec<char>>;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    summarize_patterns(input, false)
}

fn part_2(input: &str) -> usize {
    summarize_patterns(input, true)
}

fn count_reflection_diffs(pattern: &Pattern, index: usize) -> usize {
    pattern
        .iter()
        .map(|values| {
            let mut left = values.to_vec();
            let right = left.split_off(index);
            left.reverse();
            iter::zip(left, right).filter(|(a, b)| a != b).count()
        })
        .sum()
}

fn find_reflection_index(pattern: &Pattern, allowed_diffs: usize) -> Option<usize> {
    let end_index = pattern[0].len();
    (1..end_index).find(|index| count_reflection_diffs(pattern, *index) == allowed_diffs)
}

fn parse_patterns(input: &str) -> Vec<Pattern> {
    let mut lines = input.lines().peekable();
    let mut result = Vec::new();

    while lines.peek().is_some() {
        let pattern = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect();

        result.push(pattern);
    }

    result
}

fn rotate_pattern(pattern: &Pattern) -> Pattern {
    let row_count = pattern.len();
    let col_count = pattern[0].len();

    (0..col_count)
        .map(|col| (0..row_count).map(|row| pattern[row][col]).collect())
        .collect()
}

fn summarize_patterns(input: &str, allow_smudges: bool) -> usize {
    let patterns = parse_patterns(input);
    let allowed_reflection_diffs = if allow_smudges { 1 } else { 0 };

    let cols_left_of_vertical_reflection: usize = patterns
        .iter()
        .flat_map(|pattern| find_reflection_index(pattern, allowed_reflection_diffs))
        .sum();

    let rows_above_horizontal_reflection: usize = patterns
        .iter()
        .flat_map(|pattern| {
            let pattern = rotate_pattern(pattern);
            find_reflection_index(&pattern, allowed_reflection_diffs)
        })
        .sum();

    cols_left_of_vertical_reflection + (100 * rows_above_horizontal_reflection)
}

#[test]
fn sample() {
    let input = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    assert_eq!(part_1(input), 405);
    assert_eq!(part_2(input), 400);
}
