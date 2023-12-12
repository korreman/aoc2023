use std::{fmt::{Display, Write}};

use itertools::Itertools;

use crate::util::{
    graph::GraphImpl,
    grid::{pos, Dir4, Grid},
    pathfinding::bfs,
};

pub fn run(input: &str) -> (usize, usize) {
    let mut start = pos(0, 0);
    let grid = Grid::parse(input, |p, cell| match cell {
        '|' => Cell::NS,
        '-' => Cell::EW,
        'L' => Cell::NE,
        'F' => Cell::ES,
        '7' => Cell::SW,
        'J' => Cell::WN,
        'S' => {
            start = p;
            Cell::SS
        }
        _ => Cell::GR,
    });
    let mut grid = grid.pad(1, Cell::GR);
    start += pos(1, 1);

    // Part 1
    let mut visited = grid.map(|_| false);
    let mut positions =
        [Dir4::N, Dir4::E, Dir4::S, Dir4::W].map(|dir| (start.step(dir), dir.flip()));
    let mut paths = positions.map(|p| vec![p]);
    let mut part1 = 0;

    //println!("{grid}");
    'outer: loop {
        part1 += 1;
        for ((p, prev), path) in positions.iter_mut().zip(paths.iter_mut()) {
            if let Some(next_dir) = grid[*p].next(*prev) {
                if let Some(new_p) = p.step_checked(next_dir) {
                    if visited[*p] {
                        break 'outer;
                    }
                    visited[*p] = true;
                    path.push((*p, *prev));

                    *p = new_p;
                    *prev = next_dir.flip();
                }
            }
        }
    }

    // Part 2
    // 1. Construct looping path from result.
    paths.sort_by_key(|path| path.len());
    let mut paths = paths.to_vec();
    let (a, b) = (paths.pop().unwrap(), paths.pop().unwrap());

    grid[start] = Cell::connect(a[0].1.flip(), b[0].1.flip()).unwrap();
    //println!("{grid}");

    let mut path = vec![(start, b[0].1.flip())];
    path.extend(&a);
    path.extend(
        b.iter()
            .rev()
            .map(|(p, d)| (*p, grid[*p].next(*d).unwrap())),
    );

    for p in grid.nodes() {
        if !path.iter().any(|(c, _)| *c == p) {
            grid[p] = Cell::GR;
        }
    }

    // 2. Identify which direction is clockwise.
    // This can be accomplished by scanning from one side until a part of the path is encountered.
    let y = start.y;
    let mut inner_side = None;
    for p in pos(0, y).line(&start).unwrap() {
        if let Some((pos, d)) = path.iter().find(|(pos, _)| p == *pos) {
            inner_side = Some(match (grid[*pos], *d) {
                (_, Dir4::N) => Side::Right,
                (Cell::NE, Dir4::E) => Side::Right,
                (Cell::ES, Dir4::E) => Side::Left,
                (_, Dir4::S) => Side::Left,
                _ => panic!(),
            });
            break;
        }
    }
    let inner_side = inner_side.unwrap();

    // 3. Go through the path and mark the inner borders.
    let mut grid2 = grid.map(|_| Cell2::None);
    let mut grid3 = grid.clone();
    for (p, prev) in path {
        grid2[p] = Cell2::Pipe;
        //println!("{}, {}", prev, grid[p]);
        let markpos = if inner_side == Side::Left {
            match prev {
                Dir4::N => p + pos(1, 0),
                Dir4::S => p - pos(1, 0),
                Dir4::E => p + pos(0, 1),
                Dir4::W => p - pos(0, 1),
            }
            //match (prev, grid[p]) {
            //    (Dir4::N, Cell::NS) => p + pos(1, 0),
            //    (Dir4::N, Cell::NE) => p + pos(1, 0) - pos(0, 1),
            //    (Dir4::N, Cell::WN) => p + pos(1, 1),
            //    (Dir4::E, Cell::NE) => p + pos(0, 1) - pos(1, 0),
            //    (Dir4::E, Cell::ES) => p + pos(1, 1),
            //    (Dir4::E, Cell::EW) => p + pos(0, 1),
            //    (Dir4::S, Cell::NS) => p - pos(1, 0),
            //    (Dir4::S, Cell::ES) => p - pos(1, 1),
            //    (Dir4::S, Cell::SW) => p + pos(0, 1) - pos(1, 0),
            //    (Dir4::W, Cell::SW) => p + pos(1, 0) - pos(0, 1),
            //    (Dir4::W, Cell::WN) => p - pos(1, 1),
            //    (Dir4::W, Cell::EW) => p - pos(0, 1),
            //    _ => panic!(),
            //}
        } else {
            match prev {
                Dir4::N => p - pos(1, 0),
                Dir4::S => p + pos(1, 0),
                Dir4::E => p - pos(0, 1),
                Dir4::W => p + pos(0, 1),
            }
            //match (prev, grid[p]) {
            //    (Dir4::N, Cell::NS) => p - pos(1, 0),
            //    (Dir4::N, Cell::NE) => p - pos(1, 0) + pos(0, 1),
            //    (Dir4::N, Cell::WN) => p - pos(1, 1),
            //    (Dir4::E, Cell::NE) => p - pos(0, 1) + pos(1, 0),
            //    (Dir4::E, Cell::ES) => p - pos(1, 1),
            //    (Dir4::E, Cell::EW) => p - pos(0, 1),
            //    (Dir4::S, Cell::NS) => p + pos(1, 0),
            //    (Dir4::S, Cell::ES) => p + pos(1, 1),
            //    (Dir4::S, Cell::SW) => p - pos(0, 1) + pos(1, 0),
            //    (Dir4::W, Cell::SW) => p - pos(1, 0) + pos(0, 1),
            //    (Dir4::W, Cell::WN) => p + pos(1, 1),
            //    (Dir4::W, Cell::EW) => p + pos(0, 1),
            //    _ => panic!(),
            //}
        };
        if grid2[markpos] != Cell2::Pipe {
            grid2[markpos] = Cell2::Inner;
        }
        let next = markpos.step(prev);
        if grid2[next] != Cell2::Pipe {
            grid2[next] = Cell2::Inner;
        }
    }
    //println!("{grid2}");
    //println!("{grid3}");

    // 4. Flood fill and count marked nodes.
    let mut inners = vec![];
    for p in grid2.nodes() {
        if grid2[p] == Cell2::Inner {
            bfs(
                &grid2,
                p,
                |_, other| grid2[other] == Cell2::None,
                |_, cell| {
                    inners.push(cell);
                    grid3[cell] = Cell::SS;
                    //println!("\x1B[2J\x1B[H");
                    //println!("{grid3}");
                    //std::thread::sleep_ms(5);
                    false
                },
            );
        }
    }
    //println!("{grid3}");
    inners.sort();
    inners.dedup();
    let part2 = inners.len();

    (part1, part2)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell2 {
    None,
    Pipe,
    Inner,
}

impl Display for Cell2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Cell2::None => ' ',
            Cell2::Pipe => 'p',
            Cell2::Inner => 'I',
        })
    }
}

#[derive(Clone, Copy)]
enum Cell {
    NS,
    EW,
    NE,
    ES,
    SW,
    WN,
    GR,
    SS,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Cell::NS => '║',
            Cell::EW => '═',
            Cell::NE => '╚',
            Cell::ES => '╔',
            Cell::SW => '╗',
            Cell::WN => '╝',
            Cell::GR => 'x',
            Cell::SS => '█',
        };
        f.write_char(c)
    }
}

impl Cell {
    fn next(&self, prev: Dir4) -> Option<Dir4> {
        match self {
            Cell::NS => match prev {
                Dir4::N => Some(Dir4::S),
                Dir4::S => Some(Dir4::N),
                _ => None,
            },
            Cell::EW => match prev {
                Dir4::E => Some(Dir4::W),
                Dir4::W => Some(Dir4::E),
                _ => None,
            },
            Cell::NE => match prev {
                Dir4::N => Some(Dir4::E),
                Dir4::E => Some(Dir4::N),
                _ => None,
            },
            Cell::ES => match prev {
                Dir4::E => Some(Dir4::S),
                Dir4::S => Some(Dir4::E),
                _ => None,
            },
            Cell::SW => match prev {
                Dir4::S => Some(Dir4::W),
                Dir4::W => Some(Dir4::S),
                _ => None,
            },
            Cell::WN => match prev {
                Dir4::W => Some(Dir4::N),
                Dir4::N => Some(Dir4::W),
                _ => None,
            },
            _ => None,
        }
    }

    fn connect(a: Dir4, b: Dir4) -> Option<Self> {
        Some(match (a, b) {
            (Dir4::N, Dir4::E) | (Dir4::E, Dir4::N) => Self::NE,
            (Dir4::N, Dir4::S) | (Dir4::S, Dir4::N) => Self::NS,
            (Dir4::N, Dir4::W) | (Dir4::W, Dir4::N) => Self::WN,
            (Dir4::E, Dir4::S) | (Dir4::S, Dir4::E) => Self::ES,
            (Dir4::E, Dir4::W) | (Dir4::W, Dir4::E) => Self::EW,
            (Dir4::S, Dir4::W) | (Dir4::W, Dir4::S) => Self::SW,
            _ => return None,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample1() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!(super::run(input), (4, 1));
    }

    #[test]
    fn sample2() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(super::run(input).1, 10);
    }
}
