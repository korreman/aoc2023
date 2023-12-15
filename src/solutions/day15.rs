pub fn run(input: &str) -> (usize, usize) {
    let part1 = input
        .trim()
        .split(',')
        .map(|h| hash(h.as_bytes()) as usize)
        .sum();

    let mut boxes: [Vec<(&str, u8)>; 256] = [(); 256].map(|_| vec![]);
    for instr in input.trim().split(',') {
        let (label, length) = instr.split_once(['-', '=']).unwrap();
        let h = hash(label.as_bytes());
        if instr.contains('-') {
            if let Some(i) = boxes[h as usize].iter().position(|(l, _)| *l == label) {
                boxes[h as usize].remove(i);
            }
        } else {
            let length = length.parse::<u8>().unwrap();
            if let Some(i) = boxes[h as usize].iter().position(|(l, _)| *l == label) {
                boxes[h as usize][i].1 = length;
            } else {
                boxes[h as usize].push((label, length));
            }
        }
    }

    let mut part2 = 0;
    for (i, b) in boxes.iter().enumerate() {
        let a = i + 1;
        for (j, (_, length)) in b.iter().enumerate() {
            part2 += a * (j + 1) * *length as usize;
        }
    }
    (part1, part2)
}

fn hash(seq: &[u8]) -> u8 {
    seq.iter()
        .cloned()
        .fold(0, |acc, e| acc.wrapping_add(e).wrapping_mul(17))
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(super::run(input), (1320, 145));
    }
}
