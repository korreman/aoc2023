use crate::util::parsers::Numbers;
use itertools::Itertools;

pub fn run(input: &str) -> (u64, u64) {
    // Parse
    let (times, dists) = input.split_once('\n').unwrap();
    let times = Numbers::<u64>::new(times);
    let dists = Numbers::<u64>::new(dists);

    // Part 1
    let mut part1 = 1;
    for (time, dist) in times.zip(dists) {
        part1 *= ways_to_beat(time, dist);
    }

    // Part 2
    let input: String = input
        .chars()
        .filter(|x| x.is_ascii_digit() || *x == '\n')
        .collect();
    let (time, dist): (u64, u64) = Numbers::new(&input).collect_tuple().unwrap();
    let part2 = ways_to_beat(time, dist);

    (part1, part2)
}

#[inline(always)]
fn ways_to_beat(time: u64, dist: u64) -> u64 {
    let (t, d) = (time as f64, dist as f64);
    (t.powi(2) - 4.0 * d).sqrt().ceil() as u64 + 1
}
