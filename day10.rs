use std::collections::HashSet;
use std::io::{self, Read};

type Cell = (usize, usize); // row, column
type Direction = (isize, isize); // row offset, column offset
type Grid = Vec<Vec<char>>;

const DIR_N: Direction = (-1, 0);
const DIR_S: Direction = (1, 0);
const DIR_E: Direction = (0, 1);
const DIR_W: Direction = (0, -1);

// As you move in a given direction, which tiles connect with the tile you're on?
const CONNECTING_TILES_N: [char; 4] = ['S', '|', 'F', '7'];
const CONNECTING_TILES_S: [char; 4] = ['S', '|', 'L', 'J'];
const CONNECTING_TILES_E: [char; 4] = ['S', '-', 'J', '7'];
const CONNECTING_TILES_W: [char; 4] = ['S', '-', 'L', 'F'];

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let grid = parse_grid(input);
    get_loop_path(&grid).len() / 2
}

fn part_2(input: &str) -> usize {
    let grid = parse_grid(input);
    let loop_path_set: HashSet<_> = get_loop_path(&grid).into_iter().collect();

    // Insight: To determine whether a cell is inside the loop, we can scan each row left-to-right.
    // The first path cell we find that connects to the one above it is an exterior wall. All cells
    // after it are inside the loop. The next time we find a path cell that connects to the one
    // above it, that's the matching exterior wall, and all subsequent cells are outside the loop.
    // There may be multiple pairs of matching walls in a row.

    let mut result = 0;

    for row in 0..grid.len() {
        let mut is_inside = false;

        for col in 0..grid[0].len() {
            let cell = (row, col);

            if loop_path_set.contains(&cell) {
                if does_cell_above_connect(&grid, cell) {
                    is_inside = !is_inside;
                }

                continue;
            }

            if is_inside {
                result += 1;
            }
        }
    }

    result
}

fn does_cell_above_connect(grid: &Grid, cell: Cell) -> bool {
    let tile = get_tile(grid, cell);

    CONNECTING_TILES_S.contains(&tile)
        && get_adjacent_cell_if_connected(grid, cell, DIR_N, &CONNECTING_TILES_N).is_some()
}

fn get_adjacent_cell(
    grid: &Grid,
    (row, col): Cell,
    (row_offset, col_offset): Direction,
) -> Option<Cell> {
    let row = row.checked_add_signed(row_offset);
    let col = col.checked_add_signed(col_offset);

    if row.is_none() || col.is_none() {
        return None;
    }

    let row = row.unwrap();
    let col = col.unwrap();

    if row >= grid.len() || col >= grid[0].len() {
        return None;
    }

    Some((row, col))
}

fn get_adjacent_cell_if_connected(
    grid: &Grid,
    cell: Cell,
    direction: Direction,
    connecting_tiles: &[char],
) -> Option<Cell> {
    if let Some(adjacent_cell) = get_adjacent_cell(grid, cell, direction) {
        let tile = get_tile(grid, adjacent_cell);

        if connecting_tiles.contains(&tile) {
            return Some(adjacent_cell);
        }
    }

    None
}

fn get_connected_cells(grid: &Grid, cell: Cell) -> Vec<Cell> {
    match get_tile(grid, cell) {
        '|' => vec![
            get_adjacent_cell(grid, cell, DIR_N).unwrap(),
            get_adjacent_cell(grid, cell, DIR_S).unwrap(),
        ],
        '-' => vec![
            get_adjacent_cell(grid, cell, DIR_E).unwrap(),
            get_adjacent_cell(grid, cell, DIR_W).unwrap(),
        ],
        'L' => vec![
            get_adjacent_cell(grid, cell, DIR_N).unwrap(),
            get_adjacent_cell(grid, cell, DIR_E).unwrap(),
        ],
        'J' => vec![
            get_adjacent_cell(grid, cell, DIR_N).unwrap(),
            get_adjacent_cell(grid, cell, DIR_W).unwrap(),
        ],
        '7' => vec![
            get_adjacent_cell(grid, cell, DIR_S).unwrap(),
            get_adjacent_cell(grid, cell, DIR_W).unwrap(),
        ],
        'F' => vec![
            get_adjacent_cell(grid, cell, DIR_S).unwrap(),
            get_adjacent_cell(grid, cell, DIR_E).unwrap(),
        ],
        'S' => [
            (DIR_N, CONNECTING_TILES_N),
            (DIR_S, CONNECTING_TILES_S),
            (DIR_E, CONNECTING_TILES_E),
            (DIR_W, CONNECTING_TILES_W),
        ]
        .iter()
        .filter_map(|(direction, connecting_tiles)| {
            get_adjacent_cell_if_connected(grid, cell, *direction, connecting_tiles)
        })
        .collect(),
        _ => panic!(),
    }
}

fn get_loop_path(grid: &Grid) -> Vec<Cell> {
    let mut result = Vec::new();
    let mut cell = get_start_cell(grid);

    loop {
        result.push(cell);

        let next_cell = get_connected_cells(grid, cell)
            .into_iter()
            .find(|c| !result.contains(c));

        if let Some(next_cell) = next_cell {
            cell = next_cell;
        } else {
            break;
        }
    }

    result
}

fn get_start_cell(grid: &Grid) -> Cell {
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'S' {
                return (row, col);
            }
        }
    }

    panic!();
}

fn get_tile(grid: &Grid, (row, col): Cell) -> char {
    grid[row][col]
}

fn parse_grid(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[test]
fn sample() {
    let input = r".....
.S-7.
.|.|.
.L-J.
.....
";

    assert_eq!(part_1(input), 4);

    let input = r"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

    assert_eq!(part_1(input), 8);

    let input = r"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    assert_eq!(part_2(input), 4);

    let input = r"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

    assert_eq!(part_2(input), 4);

    let input = r".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    assert_eq!(part_2(input), 8);

    let input = r"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    assert_eq!(part_2(input), 10);
}
