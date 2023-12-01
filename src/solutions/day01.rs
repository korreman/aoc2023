pub fn run(input: &str) -> (u32, u32) {
    let part1 = input
        .lines()
        .map(|l| {
            let mut bytes = l.as_bytes().iter();
            let x = bytes.find(|b| b.is_ascii_digit()).unwrap();
            let mut bytes = l.as_bytes().iter();
            let y = bytes.rfind(|b| b.is_ascii_digit()).unwrap();
            ((x - b'0') * 10 + y - b'0') as u32
        })
        .sum();

    let mut part2 = 0;
    for mut line in input.lines() {
        let x = 'outer: loop {
            for (prefix, num) in NUMBERS {
                if line.strip_prefix(prefix).is_some() {
                    break 'outer num;
                }
            }
            line = &line[1..];
        };
        let y = 'outer: loop {
            for (suffix, num) in NUMBERS {
                if line.strip_suffix(suffix).is_some() {
                    break 'outer num;
                }
            }
            line = &line[..line.len() - 1];
        };
        part2 += x * 10 + y;
    }

    (part1, part2)
}

const NUMBERS: [(&str, u32); 20] = [
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];
