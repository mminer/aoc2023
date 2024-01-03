use std::cmp;
use std::io::{self, Read};

type CubeValues = (usize, usize, usize); // red, green, blue

const CUBES_R: usize = 12;
const CUBES_G: usize = 13;
const CUBES_B: usize = 14;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    parse_games(input)
        .into_iter()
        .filter(|(_, values)| is_game_possible(values))
        .map(|(id, _)| id)
        .sum()
}

fn part_2(input: &str) -> usize {
    parse_games(input)
        .into_iter()
        .map(|(_, values)| {
            let (r, g, b) = calculate_minimum_cubes_needed(values);
            r * g * b
        })
        .sum()
}

fn calculate_minimum_cubes_needed(values: Vec<CubeValues>) -> CubeValues {
    let mut minimum_r = 0;
    let mut minimum_g = 0;
    let mut minimum_b = 0;

    for (r, g, b) in values {
        minimum_r = cmp::max(minimum_r, r);
        minimum_g = cmp::max(minimum_g, g);
        minimum_b = cmp::max(minimum_b, b);
    }

    (minimum_r, minimum_g, minimum_b)
}

fn is_game_possible(values: &[CubeValues]) -> bool {
    values
        .iter()
        .all(|(r, g, b)| r <= &CUBES_R && g <= &CUBES_G && b <= &CUBES_B)
}

fn parse_games(input: &str) -> Vec<(usize, Vec<CubeValues>)> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(": ").unwrap();
            let id = left.split_whitespace().last().unwrap().parse().unwrap();

            let values = right
                .split("; ")
                .map(|values_str| {
                    let mut r = 0;
                    let mut g = 0;
                    let mut b = 0;

                    for cube_str in values_str.split(", ") {
                        let (number_str, color_str) = cube_str.split_once(' ').unwrap();
                        let number = number_str.parse().unwrap();

                        match color_str {
                            "red" => r = number,
                            "green" => g = number,
                            "blue" => b = number,
                            _ => panic!(),
                        }
                    }

                    (r, g, b)
                })
                .collect();

            (id, values)
        })
        .collect()
}

#[test]
fn sample() {
    let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    assert_eq!(part_1(input), 8);
    assert_eq!(part_2(input), 2286);
}
