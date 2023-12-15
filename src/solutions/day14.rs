use std::fmt::{Display, Write};

use itertools::Itertools;

use crate::util::{
    cycles::CycleFinder,
    graph::GraphImpl,
    grid::{Dir4, Grid},
};

pub fn run(input: &str) -> (usize, usize) {
    let mut grid = Grid::parse(input, |_, c| match c {
        'O' => Cell::Rock,
        '#' => Cell::Wall,
        _ => Cell::Empty,
    });
    let mut grid2 = grid.clone();

    roll(&mut grid, Dir4::N);
    let part1 = weight(&grid);

    let mut cycle_finder = CycleFinder::new();
    let rounds = 1_000_000_000;
    let mut i = 0;
    let part2 = loop {
        i += 1;
        for dir in [Dir4::N, Dir4::W, Dir4::S, Dir4::E] {
            roll(&mut grid2, dir);
        }
        if let Some(cycle) = cycle_finder.push(grid2.clone()) {
            let length = cycle.len();
            break weight(&cycle[(rounds - i - 1) % length]);
        }
    };
    (part1, part2)
}

// Rolls all of the rocks in a specific direction, returning their load on the north support beams.
fn roll(grid: &mut Grid<Cell>, dir: Dir4) {
    let limit = match dir {
        Dir4::N | Dir4::S => grid.width(),
        Dir4::E | Dir4::W => grid.height(),
    };

    for a in 0..limit {
        let line = match dir {
            Dir4::N => grid.col(a).collect_vec(),
            Dir4::S => grid.col(a).rev().collect_vec(),
            Dir4::E => grid.row(a).rev().collect_vec(),
            Dir4::W => grid.row(a).collect_vec(),
        };
        for mut p in line {
            if grid[p] != Cell::Rock {
                continue;
            }
            grid[p] = Cell::Empty;
            while let Some(new_p) = p.step_checked(dir) {
                if grid.get(new_p) == Some(&Cell::Empty) {
                    p = new_p;
                } else {
                    break;
                }
            }
            grid[p] = Cell::Rock;
        }
    }
}

fn weight(grid: &Grid<Cell>) -> usize {
    let mut res = 0;
    for p in grid.nodes() {
        if grid[p] == Cell::Rock {
            res += grid.height() - p.y;
        }
    }
    res
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Cell {
    Rock,
    Wall,
    Empty,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Cell::Rock => 'O',
            Cell::Wall => '█',
            Cell::Empty => ' ',
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(super::run(input), (136, 64));
    }
}
