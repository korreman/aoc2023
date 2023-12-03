use std::{collections::HashMap, ops::Range};

use itertools::Itertools;

pub fn run(input: &str) -> (u32, u32) {
    // Parse
    let mut rows = input.lines().map(parse_line).collect_vec();

    // Add padding
    rows.push(Vec::new());
    for row in &mut rows {
        row.push(Elem::Symbol(usize::MAX - 1, false));
        row.push(Elem::Symbol(usize::MAX, false));
    }

    // Identify touching numbers and symbols
    let mut numbers = HashMap::new();
    let mut gears = HashMap::new();

    let mut check_and_record = |row, range: &Range<usize>, value: u32, (i, j), gear: bool| {
        if range.contains(j) {
            numbers.insert((row, range.start), value);
            if gear {
                let entry = gears.entry((i, j)).or_insert(Vec::new());
                entry.push((row, range.start, value));
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
                (Elem::Symbol(j, gear), Elem::Number { value, range, .. })
                | (Elem::Number { value, range }, Elem::Symbol(j, gear)) => {
                    check_and_record(i, range, *value, (i, j), *gear);
                }
                _ => {}
            }
            match (head1.0, head2) {
                (Elem::Number { value, range }, Elem::Symbol(j, gear)) => {
                    check_and_record(i, range, *value, (i + 1, j), *gear);
                }
                (Elem::Symbol(j, gear), Elem::Number { value, range }) => {
                    check_and_record(i + 1, range, *value, (i, j), *gear);
                }
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
            symbol => summary.push(Elem::Symbol(j, symbol == b'*')),
        }
    }
    summary
}

#[derive(Debug)]
enum Elem {
    Symbol(usize, bool),
    Number { value: u32, range: Range<usize> },
}

impl Elem {
    fn number(value: u32, start: usize, end: usize) -> Self {
        Self::Number {
            value,
            range: start.saturating_sub(1)..end + 2,
        }
    }

    fn start(&self) -> usize {
        match self {
            Elem::Symbol(j, ..) => *j,
            Elem::Number { range, .. } => range.start,
        }
    }
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
