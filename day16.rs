use std::collections::HashSet;
use std::io::{self, Read};
use std::iter;

type Cell = (usize, usize); // row, column
type Direction = (isize, isize); // row offset, column offset
type Grid = Vec<Vec<char>>;

const DIR_N: Direction = (-1, 0);
const DIR_S: Direction = (1, 0);
const DIR_E: Direction = (0, 1);
const DIR_W: Direction = (0, -1);

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let grid = parse_grid(input);
    count_energized_tiles(&grid, DIR_E, (0, 0))
}

fn part_2(input: &str) -> usize {
    let grid = parse_grid(input);
    let row_count = grid.len();
    let col_count = grid[0].len();

    iter::empty()
        .chain((0..col_count).map(|col| (DIR_N, (row_count - 1, col))))
        .chain((0..col_count).map(|col| (DIR_S, (0, col))))
        .chain((0..row_count).map(|row| (DIR_E, (row, 0))))
        .chain((0..row_count).map(|row| (DIR_W, (row, col_count - 1))))
        .map(|(start_direction, start_cell)| {
            count_energized_tiles(&grid, start_direction, start_cell)
        })
        .max()
        .unwrap()
}

fn count_energized_tiles(grid: &Grid, start_direction: Direction, start_cell: Cell) -> usize {
    let mut visited = HashSet::new();
    traverse_grid(grid, start_direction, start_cell, &mut visited);
    let energized: HashSet<_> = visited.into_iter().map(|(_, cell)| cell).collect();
    energized.len()
}

fn get_next_directions(grid: &Grid, direction: Direction, (row, col): Cell) -> Vec<Direction> {
    let tile = grid[row][col];

    match (tile, direction) {
        ('-', DIR_N | DIR_S) => vec![DIR_E, DIR_W],
        ('|', DIR_E | DIR_W) => vec![DIR_N, DIR_S],
        ('/', DIR_N) => vec![DIR_E],
        ('/', DIR_S) => vec![DIR_W],
        ('/', DIR_E) => vec![DIR_N],
        ('/', DIR_W) => vec![DIR_S],
        ('\\', DIR_N) => vec![DIR_W],
        ('\\', DIR_S) => vec![DIR_E],
        ('\\', DIR_E) => vec![DIR_S],
        ('\\', DIR_W) => vec![DIR_N],
        _ => vec![direction],
    }
}

fn parse_grid(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn traverse_grid(
    grid: &Grid,
    direction: Direction,
    cell: Cell,
    visited: &mut HashSet<(Direction, Cell)>,
) {
    if !visited.insert((direction, cell)) {
        return;
    }

    for next_direction in get_next_directions(grid, direction, cell) {
        let (row, col) = cell;
        let (row_offset, col_offset) = next_direction;
        let next_row = row.checked_add_signed(row_offset);
        let next_col = col.checked_add_signed(col_offset);

        if next_row.is_none() || next_col.is_none() {
            continue;
        }

        let next_row = next_row.unwrap();
        let next_col = next_col.unwrap();

        if next_row >= grid.len() || next_col >= grid[0].len() {
            continue;
        }

        let next_cell = (next_row, next_col);
        traverse_grid(grid, next_direction, next_cell, visited);
    }
}

#[test]
fn sample() {
    let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    assert_eq!(part_1(input), 46);
    assert_eq!(part_2(input), 51);
}
