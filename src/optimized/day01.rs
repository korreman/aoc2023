pub fn run(input: &str) -> (u32, u32) {
    let mut part1 = 0;
    let mut part2 = 0;

    let mut tracker_fwd = Tracker::new(&Tracker::FORWARD);
    let mut tracker_bck = Tracker::new(&Tracker::BACKWARD);

    for line in input.lines() {
        let (l_digit, l_word) = tracker_fwd.compute(line.bytes());
        let (r_digit, r_word) = tracker_bck.compute(line.bytes().rev());
        tracker_fwd.reset();
        tracker_bck.reset();
        part1 += l_digit * 10 + r_digit;
        part2 += l_word * 10 + r_word;
    }
    (part1, part2)
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

    fn compute(&mut self, line: impl Iterator<Item = u8>) -> (u32, u32) {
        let mut word = 0;
        for byte in line {
            let digit = byte.wrapping_sub(b'0');
            if digit < 10 {
                if word == 0 {
                    word = digit as u32;
                }
                return (digit as u32, word);
            }
            if word == 0 {
                if let Some(n) = self.advance(byte) {
                    word = n as u32;
                }
            }
        }
        panic!()
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

    #[inline(always)]
    fn reset(&mut self) {
        self.progress = [0; 9];
    }

    const FORWARD: [&'static [u8]; 9] = [
        b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
    ];

    const BACKWARD: [&'static [u8]; 9] = [
        b"eno", b"owt", b"eerht", b"ruof", b"evif", b"xis", b"neves", b"thgie", b"enin",
    ];
}

#[cfg(test)]
mod tests {
    use super::Tracker;

    #[test]
    fn single() {
        let input = "hhrldnffive7six6onefivezllprrncczseven";
        let mut tracker = Tracker::new(&Tracker::FORWARD);
        assert_eq!(tracker.compute(input.bytes()), (7, 5));
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
