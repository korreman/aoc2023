use crate::util::grid::{Grid, Pos, Line, pos};

pub fn run(input: &str) -> (u32, u64) {
    let mut galaxies = Vec::new();
    let grid = Grid::parse(input, |p, c| match c {
        '.' => false,
        '#' => {
            galaxies.push(p);
            true
        }
        _ => panic!(),
    });

    let mut vert_lines = vec![false; grid.width()];
    let mut hori_lines = vec![false; grid.height()];
    for Pos { x, y } in &galaxies {
        vert_lines[*x] = true;
        hori_lines[*y] = true;
    }

    let mut part1 = 0;
    let mut part2 = 0;
    for &a in &galaxies {
        for &b in &galaxies {
            for p in a.line(&pos(b.x, a.y)).unwrap() {
                part1 += 1;
                part2 += 1;
                if !vert_lines[p.x] {
                    part1 += 1;
                    part2 += 999_999;
                }
            }
            for p in a.line(&pos(a.x, b.y)).unwrap() {
                part1 += 1;
                part2 += 1;
                if !hori_lines[p.y] {
                    part1 += 1;
                    part2 += 999_999;
                }
            }
            part1 -= 2;
            part2 -= 2;
        }
    }
    part1 /= 2;
    part2 /= 2;

    (part1, part2)
}
