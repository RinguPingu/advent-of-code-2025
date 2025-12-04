use grid::Grid;
use std::fs;

const ACCESSIBLE_THRESHOLD: u32 = 4;

pub fn parse_input(path: &str) -> Grid<char> {
    let input = fs::read_to_string(path).unwrap();
    let cols = input.lines().next().unwrap().chars().count();

    Grid::from_vec(input.chars().collect::<Vec<char>>(), cols)
}

pub fn is_roll(char: &char) -> bool {
    *char == '@'
}

pub fn count_adjacent_rolls(grid: &Grid<char>, index: (usize, usize)) -> u32 {
    let mut adjacent_rolls = 0;

    // Check left and right
    if let Some(char) = grid.get(index.0, index.1 - 1)
        && is_roll(char)
    {
        adjacent_rolls += 1;
    }

    adjacent_rolls
}

fn main() {
    let grid = parse_input("./input/example.txt");

    let mut accessible_rolls = 0;

    for cell in grid.indexed_iter() {
        if !is_roll(cell.1) {
            continue;
        }

        if count_adjacent_rolls(&grid, cell.0) < ACCESSIBLE_THRESHOLD {
            accessible_rolls += 1;
        }
    }

    println!("Accessible rolls:\t{}", accessible_rolls);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example_input_return_13() {
        let grid = parse_input("./input/example.txt");

        let mut accessible_rolls = 0;

        for cell in grid.indexed_iter() {
            if !is_roll(cell.1) {
                continue;
            }

            if count_adjacent_rolls(&grid, cell.0) < ACCESSIBLE_THRESHOLD {
                accessible_rolls += 1;
            }
        }

        assert_eq!(accessible_rolls, 13);
    }
}
