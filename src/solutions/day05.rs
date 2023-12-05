use itertools::Itertools;

use crate::util::parsers::Numbers;

pub fn run(input: &str) -> (u64, u64) {
    let (seeds, tail) = input.split_once("\n\n").unwrap();
    let maps: Vec<Map> = tail.split("\n\n").map(Map::parse).collect_vec();

    let mut part1 = u64::MAX;
    for seed in seeds.split(' ').dropping(1) {
        let mut r = seed.parse::<u64>().unwrap();
        for map in &maps {
            r = map.map(r);
        }
        part1 = part1.min(r);
    }

    let mut part2 = u64::MAX;
    for (start, len) in seeds.split(' ').dropping(1).tuples() {
        let s = start.parse::<u64>().unwrap();
        let n = len.parse::<u64>().unwrap();
        let mut ranges = vec![(s, s + n)];
        for map in &maps {
            ranges = ranges
                .iter()
                .flat_map(|range| map.map_range(*range))
                .collect();
        }
        part2 = part2.min(ranges.iter().map(|r| r.0).min().unwrap());
    }

    (part1, part2)
}

struct Map {
    ranges: Vec<MapRange>,
}

#[derive(Debug, Clone, Copy)]
struct MapRange {
    src: u64,
    dst: u64,
    len: u64,
}

impl Map {
    fn parse(input: &str) -> Self {
        let (_, tail) = input.split_once('\n').unwrap();
        let mut ranges = Numbers::new(tail)
            .tuples()
            .map(|(dst, src, len)| MapRange { src, dst, len })
            .collect_vec();
        ranges.sort_by_key(|r| r.src);
        Map { ranges }
    }

    fn map(&self, x: u64) -> u64 {
        let range = self.ranges.binary_search_by_key(&x, |r| r.src);
        let range = match range {
            Ok(i) => i,
            Err(i) => {
                if i == 0 {
                    return x;
                }
                i - 1
            }
        };
        let range = self.ranges[range];
        let delta = x - range.src;
        if delta < range.len {
            delta + range.dst
        } else {
            x
        }
    }

    fn map_range(&self, (mut a, b): (u64, u64)) -> Vec<(u64, u64)> {
        let i = self.ranges.binary_search_by_key(&a, |r| r.src + r.len);
        let mut i = match i {
            Ok(i) => i + 1,
            Err(i) => i,
        };

        let mut res = Vec::new();
        while a < b {
            // Finish if we've gone past all ranges
            if i >= self.ranges.len() {
                res.push((a, b));
                break;
            }
            let curr = &self.ranges[i];

            // Add free section
            if a < curr.src {
                let free_b = b.min(curr.src);
                res.push((a, free_b));
                a = free_b;
            }

            // Add mapped section
            if a >= curr.src && a < b {
                let delta_a = a - curr.src;
                let mapped_b = b.min(curr.src + curr.len);
                res.push((curr.dst + delta_a, (mapped_b + curr.dst) - curr.src));
                a = mapped_b;
            }

            i += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(super::run(input), (35, 46));
    }
}
