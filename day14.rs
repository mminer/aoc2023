use std::collections::HashMap;
use std::io::{self, Read};

type Platform = Vec<Vec<char>>;

const EMPTY: char = '.';
const ROUND_ROCK: char = 'O';

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let platform = parse_platform(input);
    let platform = tilt_north(platform);
    calculate_north_support_beam_load(&platform)
}

fn part_2(input: &str) -> usize {
    let platform = parse_platform(input);
    let platform = cycle(platform, 1000000000);
    calculate_north_support_beam_load(&platform)
}

fn calculate_north_support_beam_load(platform: &Platform) -> usize {
    let mut result = 0;

    for row in 0..platform.len() {
        for col in 0..platform[0].len() {
            if platform[row][col] != ROUND_ROCK {
                continue;
            }

            result += platform.len() - row;
        }
    }

    result
}

fn cycle(mut platform: Platform, times: usize) -> Platform {
    let mut cache = HashMap::new();

    for i in 1..=times {
        // North
        platform = tilt_north(platform);
        platform = rotate_platform(platform);

        // West
        platform = tilt_north(platform);
        platform = rotate_platform(platform);

        // South
        platform = tilt_north(platform);
        platform = rotate_platform(platform);

        // East
        platform = tilt_north(platform);
        platform = rotate_platform(platform);

        // Eventually we'll repeat cycles on platform configurations we already saw.
        // If the number of cycles required before we repeat lines up with the final cycle,
        // we can skip straight to that configuration.
        if let Some(cycle_number) = cache.insert(platform.clone(), i) {
            let cycles_remaining = times - i;
            let cycles_until_repeat = i - cycle_number;

            if cycles_remaining % cycles_until_repeat == 0 {
                break;
            }
        }
    }

    platform
}

fn parse_platform(input: &str) -> Platform {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn rotate_platform(mut platform: Platform) -> Platform {
    // Rotate by transposing the matrix then reversing each row.
    // We can use a simple in-place transposition algorithm since the input is a square matrix.
    // https://en.wikipedia.org/wiki/In-place_matrix_transposition#Square_matrices
    for row in 0..platform.len() {
        for col in (row + 1)..platform.len() {
            let temp = platform[row][col];
            platform[row][col] = platform[col][row];
            platform[col][row] = temp;
        }
    }

    for row in platform.iter_mut() {
        row.reverse();
    }

    platform
}

fn tilt_north(mut platform: Platform) -> Platform {
    for row in 1..platform.len() {
        for col in 0..platform[0].len() {
            if platform[row][col] != ROUND_ROCK {
                continue;
            }

            let empty_spaces_above = (0..row)
                .rev()
                .take_while(|&r| platform[r][col] == EMPTY)
                .count();

            if empty_spaces_above == 0 {
                continue;
            }

            platform[row - empty_spaces_above][col] = ROUND_ROCK;
            platform[row][col] = EMPTY;
        }
    }

    platform
}

#[test]
fn sample() {
    let input = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    {
        let platform = parse_platform(input);
        let platform = tilt_north(platform);

        assert_eq!(
            platform,
            parse_platform(
                r"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
            )
        );
    }

    [
        (
            1,
            r".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....",
        ),
        (
            2,
            r".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O",
        ),
        (
            3,
            r".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O",
        ),
    ]
    .iter()
    .for_each(|(cycle_count, expected_str)| {
        let platform = parse_platform(input);
        let platform = cycle(platform, *cycle_count);
        assert_eq!(platform, parse_platform(expected_str));
    });

    assert_eq!(part_1(input), 136);
    assert_eq!(part_2(input), 64);
}
