use crate::util::cycles::CycleFinder;
use itertools::Itertools;
use std::{collections::HashMap, io::Write};

pub fn run(input: &str) -> (u32, u128) {
    let (instructions, graph) = input.split_once("\n\n").unwrap();

    let graph: HashMap<&str, (&str, &str)> = graph
        .lines()
        .map(|l| (&l[..3], (&l[7..10], &l[12..15])))
        .collect();

    // Part 1
    let mut part1 = 0;
    let mut position = "AAA";
    for dir in instructions.trim().chars().cycle() {
        let edges = graph[position];
        position = match dir {
            'L' => edges.0,
            'R' => edges.1,
            _ => panic!(),
        };
        part1 += 1;
        if position == "ZZZ" {
            break;
        }
    }

    // Part 2
    let mut steps = 0u128;
    let mut positions = graph
        .keys()
        .cloned()
        .filter(|k| &k[2..] == "A")
        .collect_vec();
    let mut cycle_finders = positions.iter().map(|_| CycleFinder::new()).collect_vec();
    let mut cycles = positions.iter().map(|_| false).collect_vec();
    for dir in instructions.trim().chars().cycle() {
        steps += 1;
        for (position, (cycle_finder, cycle)) in positions
            .iter_mut()
            .zip(cycle_finders.iter_mut().zip(cycles.iter_mut()))
        {
            if *cycle {
            } else if let Some(found_cycle) = cycle_finder.push((*position, dir)) {
                *cycle = true;
                let pos = found_cycle.iter().position(|x| &x.0[2..] == "Z").unwrap();
                print!("{} + n * {} = ", steps + pos as u128, found_cycle.len());
                std::io::stdout().flush().unwrap();
            } else {
                let edges = graph[position];
                *position = match dir {
                    'L' => edges.0,
                    'R' => edges.1,
                    _ => panic!(),
                };
            }
        }
        if positions.iter().all(|p| &p[2..] == "Z") {
            break;
        }
    }

    (part1, steps)
}
