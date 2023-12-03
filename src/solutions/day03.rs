use std::collections::HashMap;

use itertools::Itertools;

pub fn run(input: &str) -> (u32, u32) {
    let rows = parse(input);
    let mut numbers = HashMap::new();
    let mut gears = HashMap::new();
    for (i, (row1, row2)) in rows.iter().tuple_windows().enumerate() {
        let mut row1 = row1.iter().tuple_windows::<(_, _)>();
        let mut row2 = row2.iter();
        let mut head1 = row1.next().unwrap();
        let mut head2 = row2.next().unwrap();
        loop {
            match head1 {
                (
                    Elem::Symbol(s_col, gear),
                    Elem::Number {
                        value, start_col, ..
                    },
                ) => {
                    if *s_col == start_col - 1 {
                        numbers.insert((i, start_col), value);
                        if *gear {
                            gears
                                .entry((i, s_col))
                                .or_insert(Vec::new())
                                .push((i, start_col, value));
                        }
                    }
                }
                (
                    Elem::Number {
                        value,
                        start_col,
                        end_col,
                    },
                    Elem::Symbol(s_col, gear),
                ) => {
                    if end_col + 1 == *s_col {
                        numbers.insert((i, start_col), value);
                        if *gear {
                            gears
                                .entry((i, s_col))
                                .or_insert(Vec::new())
                                .push((i, start_col, value));
                        }
                    }
                }
                _ => {}
            }
            match (head1.0, head2) {
                (
                    Elem::Number {
                        value,
                        start_col,
                        end_col,
                    },
                    Elem::Symbol(col, gear),
                ) => {
                    if *col >= start_col.saturating_sub(1) && *end_col + 1 >= *col {
                        numbers.insert((i, start_col), value);
                        if *gear {
                            gears
                                .entry((i + 1, col))
                                .or_insert(Vec::new())
                                .push((i, start_col, value));
                        }
                    }
                }
                (
                    Elem::Symbol(col, gear),
                    Elem::Number {
                        value,
                        start_col,
                        end_col,
                    },
                ) => {
                    if *col >= start_col.saturating_sub(1) && *end_col + 1 >= *col {
                        numbers.insert((i + 1, start_col), value);
                        if *gear {
                            gears.entry((i, col)).or_insert(Vec::new()).push((
                                i + 1,
                                start_col,
                                value,
                            ));
                        }
                    }
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
    let part1 = numbers.values().cloned().sum();
    let mut part2 = 0;
    for gear in gears.values_mut() {
        println!("{gear:?}");
        gear.sort_unstable();
        println!("{gear:?}");
        gear.dedup();
        println!("{gear:?}");
        println!();
        if gear.len() != 2 {
            break;
        }
        part2 += gear.iter().map(|n| n.2).product::<u32>();
    }
    (part1, part2)
}

fn parse(input: &str) -> Vec<Vec<Elem>> {
    let mut rows = input
        .lines()
        .map(|line| {
            let mut bytes = line.bytes().enumerate().peekable();
            let mut summary = Vec::new();
            while let Some((col, byte)) = bytes.next() {
                match byte {
                    b'.' => continue,
                    d if d.is_ascii_digit() => {
                        let start_col = col;
                        let mut end_col = col;
                        let mut value = (d - b'0') as u32;
                        while bytes
                            .peek()
                            .unwrap_or(&(usize::MAX/2, b'.'))
                            .1
                            .is_ascii_digit()
                        {
                            let (c, next_d) = bytes.next().unwrap();
                            end_col = c;
                            value *= 10;
                            value += (next_d - b'0') as u32;
                        }
                        summary.push(Elem::Number {
                            value,
                            start_col,
                            end_col,
                        });
                    }
                    s => summary.push(Elem::Symbol(col, s == b'*')),
                }
            }
            summary
        })
        .collect_vec();

    // Padding
    rows.push(Vec::new());
    for row in &mut rows {
        row.push(Elem::Symbol(usize::MAX/2 - 1, false));
        row.push(Elem::Symbol(usize::MAX/2, false));
    }
    rows
}

#[derive(Debug)]
enum Elem {
    Symbol(usize, bool),
    Number {
        value: u32,
        start_col: usize,
        end_col: usize,
    },
}

impl Elem {
    fn start(&self) -> usize {
        match self {
            Elem::Symbol(j, ..) => *j,
            Elem::Number { start_col, .. } => *start_col,
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
