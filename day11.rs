use std::collections::HashSet;
use std::io::{self, Read};

type Cell = (usize, usize); // row, column
type Image = Vec<Vec<char>>;

const TILE_GALAXY: char = '#';

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    sum_path_lengths_between_galaxies(input, 2)
}

fn part_2(input: &str) -> usize {
    sum_path_lengths_between_galaxies(input, 1_000_000)
}

fn calculate_distance_between_cells(
    (row_a, col_a): Cell,
    (row_b, col_b): Cell,
    expansion_factor: usize,
    rows_without_galaxies: &[usize],
    cols_without_galaxies: &[usize],
) -> usize {
    let row_range = if row_a < row_b {
        row_a..row_b
    } else {
        row_b..row_a
    };

    let col_range = if col_a < col_b {
        col_a..col_b
    } else {
        col_b..col_a
    };

    let row_expansion_crossings = rows_without_galaxies
        .iter()
        .filter(|row| row_range.contains(row))
        .count();

    let col_expansion_crossings = cols_without_galaxies
        .iter()
        .filter(|col| col_range.contains(col))
        .count();

    let row_distance = row_a.abs_diff(row_b) + row_expansion_crossings * (expansion_factor - 1);
    let col_distance = col_a.abs_diff(col_b) + col_expansion_crossings * (expansion_factor - 1);
    row_distance + col_distance
}

fn get_galaxy_cells(image: &Image) -> Vec<Cell> {
    let mut result = Vec::new();

    for row in 0..image.len() {
        for col in 0..image[0].len() {
            if image[row][col] == TILE_GALAXY {
                result.push((row, col));
            }
        }
    }

    result
}

fn get_galaxy_pairs(image: &Image) -> HashSet<(Cell, Cell)> {
    get_galaxy_cells(image)
        .into_iter()
        .flat_map(|cell_a| {
            get_galaxy_cells(image)
                .into_iter()
                .map(move |cell_b| {
                    // Sort so that collecting to a set removes duplicates.
                    if cell_a < cell_b {
                        (cell_a, cell_b)
                    } else {
                        (cell_b, cell_a)
                    }
                })
                .filter(|(cell_a, cell_b)| cell_a != cell_b)
        })
        .collect()
}

fn get_lines_without_galaxies(image: &Image) -> (Vec<usize>, Vec<usize>) {
    let row_count = image.len();
    let col_count = image[0].len();

    let rows_without_galaxies = (0..row_count)
        .filter(|&row| (0..col_count).all(|col| image[row][col] != TILE_GALAXY))
        .collect();

    let cols_without_galaxies = (0..col_count)
        .filter(|&col| (0..row_count).all(|row| image[row][col] != TILE_GALAXY))
        .collect();

    (rows_without_galaxies, cols_without_galaxies)
}

fn parse_image(input: &str) -> Image {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn sum_path_lengths_between_galaxies(input: &str, expansion_factor: usize) -> usize {
    let image = parse_image(input);
    let (rows_without_galaxies, cols_without_galaxies) = get_lines_without_galaxies(&image);

    get_galaxy_pairs(&image)
        .into_iter()
        .map(|(cell_a, cell_b)| {
            calculate_distance_between_cells(
                cell_a,
                cell_b,
                expansion_factor,
                &rows_without_galaxies,
                &cols_without_galaxies,
            )
        })
        .sum()
}

#[test]
fn sample() {
    let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    assert_eq!(sum_path_lengths_between_galaxies(input, 2), 374);
    assert_eq!(sum_path_lengths_between_galaxies(input, 10), 1030);
    assert_eq!(sum_path_lengths_between_galaxies(input, 100), 8410);
}
