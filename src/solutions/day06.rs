use itertools::Itertools;

use crate::util::parsers::Numbers;

pub fn run(input: &str) -> (u32, u32) {
    // Parse
    let (times, dists) = input
        .split_once('\n')
        .unwrap();
    let times: Vec<u32> = Numbers::new(times).collect();
    let dists: Vec<u32> = Numbers::new(dists).collect();

    // Part 1
    let mut part1 = 1;
    for (time, dist) in times.iter().zip(dists.iter()) {
        let mut hold_time = 1;
        let mut release_time = time - 1;

        let mut ways_to_beat = 0;
        while release_time > 0 {
            if hold_time * release_time > *dist {
                ways_to_beat += 1;
            }
            hold_time += 1;
            release_time -= 1;
        }
        part1 *= ways_to_beat;
    }

    // Part 2

    let input: String = input.chars().filter(|x| x.is_ascii_digit() || *x == '\n').collect();
    let (time, dist): (u64, u64) = Numbers::new(&input).collect_tuple().unwrap();

    let mut hold_time = 1;
    let mut release_time = time - 1;

    let mut part2 = 0;
    while release_time > 0 {
        if hold_time * release_time > dist {
            part2 += 1;
        }
        hold_time += 1;
        release_time -= 1;
    }

    (part1, part2)
}
