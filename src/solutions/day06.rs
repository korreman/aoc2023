use itertools::Itertools;

use crate::util::parsers::Numbers;

pub fn run(input: &str) -> (u64, u64) {
    // Parse
    let (times, dists) = input.split_once('\n').unwrap();
    let times: Vec<u64> = Numbers::new(times).collect();
    let dists: Vec<u64> = Numbers::new(dists).collect();

    // Part 1
    let mut part1 = 1;
    for (time, dist) in times.iter().zip(dists.iter()) {
        part1 *= ways_to_beat(*time, *dist);
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

fn ways_to_beat(time: u64, dist: u64) -> u64 {
    let mut hold_time = 1;
    let mut release_time = time - 1;
    let mut res = 0;
    while release_time > 0 {
        if hold_time * release_time > dist {
            res += 1;
        }
        hold_time += 1;
        release_time -= 1;
    }
    res
}
