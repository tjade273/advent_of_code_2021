use counter::Counter;
use std::str::FromStr;

const GRID_SIZE: usize = 100;

struct Grid<const N: usize> {
    heights: [[u8; N]; N],
    flows_to: [[Option<(usize, usize)>; N]; N],
}

impl<const N: usize> FromStr for Grid<N> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let heights: [[u8; N]; N] = s
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
        Ok(Self::new(heights))
    }
}

impl<const N: usize> Grid<N> {
    fn downward_neighbor(&self, i: usize, j: usize) -> (usize, usize) {
        let height = self.heights[i][j];
        if height == 9 {
            return (i, j);
        }
        for (x, y) in [
            (i.saturating_sub(1), j),
            (i + 1, j),
            (i, j.saturating_sub(1)),
            (i, j + 1),
        ] {
            if let Some(&h) = self.heights.get(x).and_then(|r| r.get(y)) {
                if h < height {
                    return (x, y);
                }
            }
        }
        (i, j)
    }

    fn compute_flow_from(&mut self, i: usize, j: usize) {
        if self.flows_to[i][j].is_some() {
            return;
        }
        self.flows_to[i][j] = Some((i, j));
        let (x, y) = self.downward_neighbor(i, j);
        self.compute_flow_from(x, y);
        self.flows_to[i][j] = self.flows_to[x][y]
    }

    fn compute_flows(&mut self) {
        for i in 0..N {
            for j in 0..N {
                self.compute_flow_from(i, j)
            }
        }
    }

    fn new(heights: [[u8; N]; N]) -> Self {
        let mut s = Self {
            heights,
            flows_to: [[None; N]; N],
        };
        s.compute_flows();
        s
    }

    fn largest_basins(&self) -> Vec<usize> {
        let counts: Counter<(usize, usize)> = self
            .flows_to
            .iter()
            .flatten()
            .copied()
            .map(Option::unwrap)
            .collect();
        counts
            .most_common_ordered()
            .iter()
            .map(|(_, cnt)| *cnt)
            .collect()
    }

    fn local_minima(&self) -> Vec<usize> {
        let mut minima = Vec::new();
        for i in 0..N {
            for j in 0..N {
                if self.flows_to[i][j] == Some((i, j)) && self.heights[i][j] != 9 {
                    minima.push(self.heights[i][j] as usize);
                }
            }
        }
        minima
    }
}

fn main() {
    let input = include_str!("input.txt");
    let grid: Grid<GRID_SIZE> = input.parse().unwrap();
    let part1: usize = grid.local_minima().iter().map(|x| x + 1).sum();
    let part2: usize = grid.largest_basins().iter().take(3).product();
    println!("{}, {}", part1, part2)
}
