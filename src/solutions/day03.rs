use std::{collections::HashMap, ops::Range};

use itertools::Itertools;

pub fn run(input: &str) -> (u32, u32) {
    // Parse
    let mut rows = input.lines().map(parse_line).collect_vec();

    // Add padding
    rows.push(Vec::new());
    for row in &mut rows {
        row.push(Elem::symbol(usize::MAX - 1, false));
        row.push(Elem::symbol(usize::MAX, false));
    }

    // Identify touching numbers and symbols
    let mut numbers = HashMap::new();
    let mut gears = HashMap::new();

    let mut check_and_record = |i, n: &Number, is, s: &Symbol| {
        if n.range.contains(&s.j) {
            numbers.insert((i, n.range.start), n.value);
            if s.gear {
                let entry = gears.entry((is, s.j)).or_insert(Vec::new());
                entry.push((i, n.range.start, n.value));
            }
        }
    };

    for (i, (row1, row2)) in rows.iter().tuple_windows().enumerate() {
        let mut row1 = row1.iter().tuple_windows::<(_, _)>();
        let mut row2 = row2.iter();
        let mut head1 = row1.next().unwrap();
        let mut head2 = row2.next().unwrap();
        loop {
            match head1 {
                (Elem::Symbol(s), Elem::Number(n)) | (Elem::Number(n), Elem::Symbol(s)) => {
                    check_and_record(i, n, i, s);
                }
                _ => {}
            }
            match (head1.0, head2) {
                (Elem::Number(n), Elem::Symbol(s)) => check_and_record(i, n, i + 1, s),
                (Elem::Symbol(s), Elem::Number(n)) => check_and_record(i + 1, n, i, s),
                _ => {}
            }
            if head1.0.start() < head2.start() {
                if let Some(new_head) = row1.next() {
                    head1 = new_head;
                } else {
                    break;
                }
            } else if let Some(new_head) = row2.next() {
                head2 = new_head;
            }
        }
    }

    // Compute results
    let part1 = numbers.values().cloned().sum();
    let mut part2 = 0;
    for gear in gears.values_mut() {
        gear.sort_unstable();
        gear.dedup();
        if gear.len() != 2 {
            continue;
        }
        part2 += gear.iter().map(|n| n.2).product::<u32>();
    }
    (part1, part2)
}

fn parse_line(line: &str) -> Vec<Elem> {
    let mut bytes = line.bytes().enumerate().peekable();
    let mut summary = Vec::new();
    while let Some((j, byte)) = bytes.next() {
        match byte {
            b'.' => continue,
            d if d.is_ascii_digit() => {
                let mut end = j;
                let mut value = (d - b'0') as u32;
                while bytes.peek().map(|b| b.1.is_ascii_digit()) == Some(true) {
                    let (c, next_d) = bytes.next().unwrap();
                    end = c;
                    value = value * 10 + (next_d - b'0') as u32;
                }
                summary.push(Elem::number(value, j, end));
            }
            symbol => summary.push(Elem::symbol(j, symbol == b'*')),
        }
    }
    summary
}

#[derive(Debug)]
enum Elem {
    Symbol(Symbol),
    Number(Number),
}

impl Elem {
    fn symbol(j: usize, gear: bool) -> Self {
        Self::Symbol(Symbol { j, gear })
    }

    fn number(value: u32, start: usize, end: usize) -> Self {
        Self::Number(Number {
            value,
            range: start.saturating_sub(1)..end + 2,
        })
    }

    fn start(&self) -> usize {
        match self {
            Elem::Symbol(s) => s.j,
            Elem::Number(n) => n.range.start,
        }
    }
}

#[derive(Debug)]
struct Symbol {
    j: usize,
    gear: bool,
}

#[derive(Debug)]
struct Number {
    range: Range<usize>,
    value: u32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(super::run(input), (4361, 467835));
    }

    #[test]
    fn corner_cases() {
        let input = "....................
.......100..........
......*.............
..........'.........
...........100......
....100.............
.......,...100......
..100.......'.......
.....-..............
..100...............
........100&........
.......&100.........
.................100
..............100...
100.................
g...................
...-....-...........
..5......5....-5....
....................
...5.....5....5-....
....-...-...........
..............-.....
....5.........5.....
....-...............
.........3........-.
..........&100...5..
";
        assert_eq!(super::run(input).0, 1048);
    }
}
