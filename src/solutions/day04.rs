use std::collections::VecDeque;

use itertools::Itertools;

use crate::util::parsers::Numbers;

pub fn run(input: &str) -> (u32, u32) {
    let mut part1 = 0;
    let mut part2 = 0;
    let mut q = VecDeque::from([1]);
    for line in input.lines() {
        let (_, winners, draws) = line.split([':', '|']).collect_tuple().unwrap();
        let winners: Vec<u32> = Numbers::new(winners).collect_vec();
        let good_draws: usize = Numbers::new(draws).filter(|draw| winners.contains(draw)).count();

        if good_draws >= 1 {
            part1 += 1 << good_draws.saturating_sub(1);
        }

        let num_tickets = q.pop_front().unwrap();
        while q.len() < good_draws + 1 {
            q.push_back(1);
        }
        for i in 0..good_draws {
            q[i] += num_tickets;
        }
        part2 += num_tickets;
    }
    (part1, part2)
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(super::run(input), (13, 30));
    }
}
