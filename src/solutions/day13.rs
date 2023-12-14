use itertools::Itertools;

use crate::util::grid::Grid;

pub fn run(input: &str) -> (usize, usize) {
    let mut part1 = 0;
    let mut part2 = 0;
    for instance in input.split("\n\n") {
        let grid = Grid::parse(instance, |_, c| c == '#');

        // TODO: Refactor to one loop instead of four.
        let mut j_mirror = 1usize;
        for j in 1..grid.width() {
            if let Some(mirror) = (2 * j_mirror).checked_sub(j + 1) {
                let a = grid.col(j).map(|p| grid[p]).collect_vec();
                let b = grid.col(mirror).map(|p| grid[p]).collect_vec();
                if a != b {
                    j_mirror = j + 1;
                }
            }
        }
        if j_mirror != grid.width() {
            part1 += j_mirror;
        }

        let mut i_mirror = 1usize;
        for i in 1..grid.height() {
            if let Some(mirror) = (2 * i_mirror).checked_sub(i + 1) {
                let a = grid.row(i).map(|p| grid[p]).collect_vec();
                let b = grid.row(mirror).map(|p| grid[p]).collect_vec();
                if a != b {
                    i_mirror = i + 1;
                }
            }
        }
        if i_mirror != grid.height() {
            part1 += i_mirror * 100;
        }

        let mut j_mirror2 = 1usize;
        let mut error_slack = 1;
        for j in 1..grid.width() {
            if let Some(mirror) = (2 * j_mirror2).checked_sub(j + 1) {
                let a = grid.col(j).map(|p| grid[p]);
                let b = grid.col(mirror).map(|p| grid[p]);
                let matches = a.zip(b).filter(|(a, b)| a != b).count();
                if matches > error_slack || j_mirror2 == j_mirror {
                    j_mirror2 = j + 1;
                    error_slack = 1;
                } else {
                    error_slack -= matches;
                }
            }
        }
        if j_mirror2 != grid.width() {
            part2 += j_mirror2;
        }

        let mut i_mirror2 = 1usize;
        let mut error_slack = 1;
        for i in 1..grid.height() {
            if let Some(mirror) = (2 * i_mirror2).checked_sub(i + 1) {
                let a = grid.row(i).map(|p| grid[p]);
                let b = grid.row(mirror).map(|p| grid[p]);
                let matches = a.zip(b).filter(|(a, b)| a != b).count();
                if matches > error_slack || i_mirror2 == i_mirror {
                    i_mirror2 = i + 1;
                    error_slack = 1;
                } else {
                    error_slack -= matches;
                }
            }
        }
        if i_mirror2 != grid.height() {
            part2 += 100 * i_mirror2;
        }
    }
    (part1, part2)
}

#[cfg(test)]
mod test {
    #[test]
    fn sample() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        assert_eq!(super::run(input), (405, 400));
    }
}
