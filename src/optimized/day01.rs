pub fn run(input: &str) -> (u32, u32) {
    let mut part1 = 0;
    let mut part2 = 0;

    for line in input.lines() {
        let (l_digit, l_word) = compute(line.bytes(), &Tracker::FORWARD);
        let (r_digit, r_word) = compute(line.bytes().rev(), &Tracker::BACKWARD);
        part1 += l_digit * 10 + r_digit;
        part2 += l_word * 10 + r_word;
    }
    (part1, part2)
}

fn compute(line: impl Iterator<Item = u8>, numbers: &'static [&'static [u8]; 9]) -> (u32, u32) {
    let mut word = 0;
    let mut tracker = Tracker::new(numbers);
    for byte in line {
        let digit = byte.wrapping_sub(b'0');
        if digit < 10 {
            if word == 0 {
                word = digit as u32;
            }
            return (digit as u32, word);
        }
        if word == 0 {
            if let Some(n) = tracker.advance(byte) {
                word = n as u32;
            }
        }
    }
    panic!()
}

struct Tracker {
    progress: [usize; 9],
    numbers: &'static [&'static [u8]; 9],
}

impl Tracker {
    fn new(numbers: &'static [&'static [u8]; 9]) -> Self {
        Self {
            progress: [0; 9],
            numbers,
        }
    }

    #[inline(always)]
    fn advance(&mut self, b: u8) -> Option<usize> {
        for (i, p) in self.progress.iter_mut().enumerate() {
            if b == self.numbers[i][*p] {
                *p += 1;
                if *p == self.numbers[i].len() {
                    return Some(i + 1);
                }
            } else if b == self.numbers[i][0] {
                *p = 1;
            } else {
                *p = 0;
            }
        }
        None
    }

    const FORWARD: [&[u8]; 9] = [
        b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
    ];

    const BACKWARD: [&[u8]; 9] = [
        b"eno", b"owt", b"eerht", b"ruof", b"evif", b"xis", b"neves", b"thgie", b"enin",
    ];
}

#[cfg(test)]
mod tests {
    use super::{compute, Tracker};

    #[test]
    fn single() {
        let input = "hhrldnffive7six6onefivezllprrncczseven";
        assert_eq!(compute(input.bytes(), &Tracker::FORWARD), (7, 5));
    }

    #[test]
    fn sample() {
        let input = "two1nine
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(super::run(input).1, 198);
    }
}
