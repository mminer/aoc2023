use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

type Cell = (usize, usize); // row, column

const GEAR_SYMBOL: char = '*';

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();
    let mut result = 0;

    for row in 0..lines.len() {
        let mut part_number = 0;
        let mut is_valid = false;

        for col in 0..lines[row].len() {
            if let Some(digit) = get_char(&lines, row, col).to_digit(10) {
                part_number = part_number * 10 + (digit as usize);

                let is_adjacent_to_symbol =
                    get_adjacent_cells(&lines, row, col)
                        .into_iter()
                        .any(|(adj_row, adj_col)| {
                            let c = get_char(&lines, adj_row, adj_col);
                            !c.is_ascii_digit() && c != '.'
                        });

                is_valid |= is_adjacent_to_symbol;
            } else {
                if is_valid {
                    result += part_number;
                }

                part_number = 0;
                is_valid = false;
            }
        }

        if is_valid {
            result += part_number;
        }
    }

    result
}

fn part_2(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();

    let mut gear_part_numbers: HashMap<_, _> = get_gears(&lines)
        .into_iter()
        .map(|cell| (cell, Vec::new()))
        .collect();

    let mut add_part_number_to_adjacent_gears = |part_number, part_adjacent_cells: &HashSet<_>| {
        for cell in part_adjacent_cells {
            if let Some(part_numbers) = gear_part_numbers.get_mut(cell) {
                part_numbers.push(part_number);
            }
        }
    };

    for row in 0..lines.len() {
        let mut part_number = 0;
        let mut part_adjacent_cells = HashSet::new();

        for col in 0..lines[row].len() {
            if let Some(digit) = get_char(&lines, row, col).to_digit(10) {
                part_number = part_number * 10 + (digit as usize);

                let adjacent_cells = get_adjacent_cells(&lines, row, col).into_iter().filter(
                    |(adj_row, adj_col)| !get_char(&lines, *adj_row, *adj_col).is_ascii_digit(),
                );

                part_adjacent_cells.extend(adjacent_cells);
            } else {
                add_part_number_to_adjacent_gears(part_number, &part_adjacent_cells);
                part_number = 0;
                part_adjacent_cells.clear();
            }
        }

        add_part_number_to_adjacent_gears(part_number, &part_adjacent_cells);
    }

    gear_part_numbers
        .iter()
        .filter(|(_, part_numbers)| part_numbers.len() == 2)
        .map(|(_, part_numbers)| part_numbers[0] * part_numbers[1])
        .sum()
}

fn get_adjacent_cells(lines: &[&str], row: usize, col: usize) -> Vec<Cell> {
    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    offsets
        .iter()
        .filter_map(|(offset_row, offset_col)| {
            let adj_row = row.checked_add_signed(*offset_row);
            let adj_col = col.checked_add_signed(*offset_col);

            if adj_row.is_none() || adj_col.is_none() {
                return None;
            }

            let adj_row = adj_row.unwrap();
            let adj_col = adj_col.unwrap();

            if adj_row >= lines.len() || adj_col >= lines[0].len() {
                return None;
            }

            Some((adj_row, adj_col))
        })
        .collect()
}

fn get_char(lines: &[&str], row: usize, col: usize) -> char {
    lines[row].chars().nth(col).unwrap()
}

fn get_gears(lines: &[&str]) -> Vec<Cell> {
    let mut result = Vec::new();

    for row in 0..lines.len() {
        for col in 0..lines[row].len() {
            if get_char(lines, row, col) == GEAR_SYMBOL {
                result.push((row, col));
            }
        }
    }

    result
}

#[test]
fn sample() {
    let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    assert_eq!(part_1(input), 4361);
    assert_eq!(part_2(input), 467835);
}
