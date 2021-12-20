use itertools::iproduct;
use std::cmp::min;
use std::iter;
const N: usize = 10;

struct Grid<const N: usize> {
    grid: [[Option<u8>; N]; N],
    flashes: usize,
}

impl<const N: usize> Grid<N> {
    fn new(grid: [[u8; N]; N]) -> Self {
        Self {
            grid: grid.map(|x| x.map(Some)),
            flashes: 0,
        }
    }

    fn flash(&mut self, x: usize, y: usize) -> usize {
        match self.grid[x][y] {
            Some(n) if n > 9 => self.grid[x][y] = None,
            _ => return 0,
        }
        for i in x.saturating_sub(1)..=min(N - 1, x + 1) {
            for j in y.saturating_sub(1)..=min(N - 1, y + 1) {
                self.grid[i][j] = self.grid[i][j].map(|n| n + 1)
            }
        }
        1
    }

    fn substep(&mut self) -> usize {
        iproduct!(0..N, 0..N).map(|(i, j)| self.flash(i, j)).sum()
    }

    fn step(&mut self) {
        self.grid.iter_mut().for_each(|r| {
            r.iter_mut()
                .for_each(|x| *x = x.map_or(Some(1), |x| Some(x + 1)))
        });
        self.flashes += iter::repeat_with(|| self.substep())
            .take_while(|&x| x > 0)
            .sum::<usize>();
    }

    fn synced(&self) -> bool {
        self.grid.iter().all(|r| r.iter().all(Option::is_none))
    }
}

fn main() {
    let input = include_str!("input.txt");
    let input_grid: [[u8; N]; N] = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let mut grid = Grid::new(input_grid);
    for i in 0.. {
        if i == 100 {
            println!("Part1: {}", grid.flashes);
        }
        if grid.synced() {
            println!("Part2: {}", i);
            break;
        }
        grid.step()
    }
}
