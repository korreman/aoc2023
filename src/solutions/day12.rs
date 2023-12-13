use std::collections::HashMap;
use itertools::Itertools;
use crate::util::parsers::Numbers;

pub fn run(input: &str) -> (u128, u128) {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        // Parse
        let (row, summary) = line.split_once(' ').unwrap();
        let row = row
            .as_bytes()
            .iter()
            .map(|b| match b {
                b'.' => Cell::Nothin,
                b'#' => Cell::Spring,
                b'?' => Cell::Damage,
                _ => panic!("failed to parse row summary"),
            })
            .collect_vec();
        let summary = Numbers::<usize>::new(summary).collect_vec();

        // Part 1
        let mut solver = Solver {
            row: &row,
            summary: &summary,
            memory: HashMap::new(),
        };
        part1 += solver.solve(0, 0);

        // Part 2
        let mut row2 = Vec::new();
        let mut summary2: Vec<usize> = Vec::new();
        for _ in 0..5 {
            row2.extend(row.iter());
            row2.push(Cell::Damage);
            summary2.extend(summary.iter());
        }
        row2.pop();
        let mut solver = Solver {
            row: &row2,
            summary: &summary2,
            memory: HashMap::new(),
        };
        part2 += solver.solve(0, 0);
    }
    (part1, part2)
}

#[derive(Eq, PartialEq, Clone, Copy)]
enum Cell {
    Nothin,
    Spring,
    Damage,
}

struct Solver<'a> {
    // TODO: Trim the end and remove costly base case.
    row: &'a [Cell],
    summary: &'a [usize],
    memory: HashMap<(usize, usize), u128>,
}

impl<'a> Solver<'a> {
    fn solve(&mut self, row_idx: usize, summary_idx: usize) -> u128 {
        // Memoization lookup
        if let Some(res) = self.memory.get(&(row_idx, summary_idx)) {
            return *res;
        }

        // Base cases
        if row_idx >= self.row.len() {
            if summary_idx >= self.summary.len() {
                return 1;
            } else {
                return 0;
            }
        }
        if summary_idx >= self.summary.len() {
            if self.row[row_idx..].iter().any(|c| *c == Cell::Spring) {
                return 0;
            } else {
                return 1;
            }
        }
        let piece = self.summary[summary_idx];
        if row_idx + piece > self.row.len() {
            return 0;
        }

        // Compute
        let mut res = 0;
        let cell = self.row[row_idx];
        if cell != Cell::Spring {
            res += self.solve(row_idx + 1, summary_idx);
        }
        if cell != Cell::Nothin {
            let fits = self.row[row_idx..row_idx + piece]
                .iter()
                .all(|c| *c != Cell::Nothin);
            let fits = fits
                && self
                    .row
                    .get(row_idx + piece)
                    .map(|c| *c != Cell::Spring)
                    .unwrap_or(true);
            if fits {
                res += self.solve(row_idx + piece + 1, summary_idx + 1);
            }
        }
        // Memoization store
        self.memory.insert((row_idx, summary_idx), res);
        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(super::run(input).0, 21);
    }
}
