use itertools::Itertools;

use crate::util::parsers::Numbers;

pub fn run(input: &str) -> (i32, i32) {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        let sequence = Numbers::new(line).collect_vec();
        let (first, last) = task(&sequence);
        part1 += last;
        part2 += first;
    }
    (part1, part2)
}

fn task(data: &[i32]) -> (i32, i32) {
    if data.iter().all(|x| *x == 0) {
        return (0, 0);
    }
    let diffs = data
        .iter()
        .tuple_windows()
        .map(|(a, b)| *b - *a)
        .collect_vec();
    let (d1, d2) = task(&diffs);
    (data[0] - d1, *data.last().unwrap() + d2)
}
