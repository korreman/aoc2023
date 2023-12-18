use std::fmt::{Display, Write};

use itertools::Itertools;

use crate::util::{
    grid::{pos, Dir4, Grid},
    math::gcd,
    pathfinding::bfs,
};

pub fn run(input: &str) -> (usize, usize) {
    let mut insts2 = Vec::new();
    let insts = input
        .lines()
        .map(|l| {
            let (dir, len, color) = l.split_ascii_whitespace().collect_tuple().unwrap();
            let dir = match dir {
                "U" => Dir4::N,
                "D" => Dir4::S,
                "L" => Dir4::W,
                "R" => Dir4::E,
                _ => panic!(),
            };
            let len = len.parse::<usize>().unwrap();
            let dir2 = match color.chars().nth(7).unwrap() {
                '0' => Dir4::E,
                '1' => Dir4::S,
                '2' => Dir4::W,
                '3' => Dir4::N,
                _ => panic!(),
            };
            let len2 = usize::from_str_radix(&color[2..7], 16).unwrap();
            insts2.push((dir2, len2));
            (dir, len)
        })
        .collect_vec();

    let part1 = solve(insts);
    let part2 = solve(insts2);

    // Time to make a 2-dimensional range map again?
    (part1, part2)
}

fn solve(insts: Vec<(Dir4, usize)>) -> usize {
    let (mut x, mut y) = (0i64, 0i64);
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (0, 0, 0, 0);

    for (d, n) in &insts {
        match d {
            Dir4::N => y -= *n as i64,
            Dir4::E => x += *n as i64,
            Dir4::S => y += *n as i64,
            Dir4::W => x -= *n as i64,
        }
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;
    let mut grid = Grid::new_filled(width, height, false);

    let mut res = 0;
    let mut p = pos(-min_x as usize, -min_y as usize);
    for (d, mut n) in insts {
        while n > 0 {
            res += 1;
            grid[p] = true;
            p = p.step(d);
            n -= 1;
        }
    }

    // Identify a side `#.` of the structure, where the `#` is visible from the edge.
    let mut start = pos(0, -min_y as usize);
    loop {
        if grid[start] {
            if !grid[start.step(Dir4::E)] {
                start = start.step(Dir4::E);
                break;
            } else {
                start = pos(0, start.y + 1);
            }
        } else {
            start = start.step(Dir4::E);
        }
    }
    // Flood fill from that `.`.
    let mut grid2 = grid.clone();
    bfs(
        &grid,
        start,
        |_, p| !grid[p],
        |_, p| {
            res += 1;
            grid2[p] = true;
            false
        },
    );
    res
}

enum Cell {
    T,
    F,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Cell::T => '#',
            Cell::F => '.',
        };
        f.write_char(c)
    }
}
