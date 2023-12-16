use std::mem::swap;

use itertools::Itertools;

use crate::util::{
    graph::GraphImpl,
    grid::{pos, Dir4, Grid, Pos, Rot},
};

pub fn run(input: &str) -> (usize, usize) {
    let grid = Grid::parse(input, |_, c| match c {
        '.' => Cell::Empty,
        '/' => Cell::Slash,
        '\\' => Cell::Bslas,
        '|' => Cell::Verti,
        '-' => Cell::Horiz,
        _ => panic!(),
    });

    let part1 = solve(&grid, pos(0, 0), Dir4::E);
    let mut part2 = 0;

    for p in grid.row(0) {
        part2 = part2.max(solve(&grid, p, Dir4::S));
    }
    for p in grid.row(grid.height() - 1) {
        part2 = part2.max(solve(&grid, p, Dir4::N));
    }
    for p in grid.col(0) {
        part2 = part2.max(solve(&grid, p, Dir4::E));
    }
    for p in grid.col(grid.width() - 1) {
        part2 = part2.max(solve(&grid, p, Dir4::W));
    }

    (part1, part2)
}

fn solve(grid: &Grid<Cell>, p: Pos, d: Dir4) -> usize {
    let mut surface = grid.map(|_| [false; 4]);
    let mut beams = vec![(p, d)];
    let mut beams_off = vec![];
    while !beams.is_empty() {
        for (p, d) in &beams {
            surface[*p][dir2idx(*d)] = true;
            match (grid[*p], d) {
                (Cell::Empty, _)
                | (Cell::Verti, Dir4::N)
                | (Cell::Verti, Dir4::S)
                | (Cell::Horiz, Dir4::W)
                | (Cell::Horiz, Dir4::E) => beams_off.push((p.step(*d), *d)),

                (Cell::Slash, _) => {
                    let d = d.flip_x().rotate(Rot::R);
                    beams_off.push((p.step(d), d))
                }
                (Cell::Bslas, _) => {
                    let d = d.flip_x().rotate(Rot::L);
                    beams_off.push((p.step(d), d))
                }

                (Cell::Verti, Dir4::E) | (Cell::Verti, Dir4::W) => {
                    beams_off.push((p.step(Dir4::N), Dir4::N));
                    beams_off.push((p.step(Dir4::S), Dir4::S));
                }

                (Cell::Horiz, Dir4::N) | (Cell::Horiz, Dir4::S) => {
                    beams_off.push((p.step(Dir4::W), Dir4::W));
                    beams_off.push((p.step(Dir4::E), Dir4::E));
                }
            }
        }
        beams_off = beams_off
            .iter()
            .cloned()
            .filter(|(p, d)| surface.contains(*p) && !surface[*p][dir2idx(*d)])
            .collect_vec();
        swap(&mut beams, &mut beams_off);
        beams_off.clear();
    }
    surface
        .nodes()
        .filter(|p| surface[*p].iter().any(|x| *x))
        .count()
}

fn dir2idx(dir: Dir4) -> usize {
    match dir {
        Dir4::N => 0,
        Dir4::E => 1,
        Dir4::S => 2,
        Dir4::W => 3,
    }
}

#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Slash,
    Bslas,
    Verti,
    Horiz,
}
