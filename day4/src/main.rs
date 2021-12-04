use std::convert::TryInto;

const GRID_N: usize = 5;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cell {
    Marked,
    Unmarked(u8),
}

fn parse_called_numbers(input: &str) -> Vec<u8> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

fn parse_grid(input: &str) -> [[Cell; GRID_N]; GRID_N] {
    input
        .lines()
        .map(|l| {
            l.split(' ')
                .filter(|x| !x.is_empty())
                .map(|n| Cell::Unmarked(n.parse().unwrap()))
                .collect::<Vec<Cell>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[Cell; GRID_N]>>()
        .try_into()
        .unwrap()
}

impl Cell {
    fn mark_if_equal(self, n: u8) -> Self {
        match self {
            Self::Unmarked(x) if x == n => Self::Marked,
            c => c,
        }
    }
}

fn mark_grid(grid: &mut [[Cell; GRID_N]; GRID_N], n: u8) {
    for row in grid {
        for cell in row {
            *cell = cell.mark_if_equal(n)
        }
    }
}

fn bingo(grid: &[[Cell; GRID_N]; GRID_N]) -> bool {
    let row_bingo = grid
        .iter()
        .any(|row| row.iter().all(|&c| c == Cell::Marked));
    let col_bingo =
        (0..GRID_N).any(|col| grid.iter().map(|row| row[col]).all(|c| c == Cell::Marked));
    row_bingo || col_bingo
}

fn final_score(grid: &[[Cell; GRID_N]; GRID_N], last_number: u8) -> usize {
    let unmarked_sum = grid.iter().fold(0usize, |acc, row| {
        row.iter().fold(acc, |acc, &c| match c {
            Cell::Unmarked(n) => acc + (n as usize),
            _ => acc,
        })
    });
    unmarked_sum * (last_number as usize)
}

fn part1(mut grids: Vec<[[Cell; GRID_N]; GRID_N]>, called: Vec<u8>) -> usize {
    let mut n = 0;
    let mut called_iter = called.iter();
    while !grids.iter().any(|g| bingo(g)) {
        n = *called_iter.next().unwrap();
        grids.iter_mut().for_each(|g| mark_grid(g, n))
    }
    let bingo_grid = grids.iter().find(|g| bingo(g)).unwrap();
    final_score(bingo_grid, n)
}

fn part2(mut grids: Vec<[[Cell; GRID_N]; GRID_N]>, called: Vec<u8>) -> usize {
    let mut n = 0;
    let mut called_iter = called.iter();
    let mut bingo_grid = grids[0];
    while !grids.is_empty() {
        n = *called_iter.next().unwrap();
        grids.iter_mut().for_each(|g| mark_grid(g, n));
        if grids.len() == 1 && bingo(&grids[0]) {
            bingo_grid = grids[0]
        }
        grids.retain(|g| !bingo(g));
    }
    println!("Final n: {}", n);
    println!("Final grid: {:?}", bingo_grid);
    final_score(&bingo_grid, n)
}

fn run_part(input: &str, first: bool) -> usize {
    let (called_str, grids_str) = input.split_once("\n\n").unwrap();
    let called = parse_called_numbers(called_str);
    let grids = grids_str.split("\n\n").map(parse_grid).collect();
    if first {
        part1(grids, called)
    } else {
        part2(grids, called)
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}, {}", run_part(input, true), run_part(input, false));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("test.txt");
        assert!(run_part(input, true) == 4512)
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("test.txt");
        assert!(run_part(input, false) == 1924)
    }
}
