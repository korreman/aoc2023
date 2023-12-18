pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a.rem_euclid(b);
        a = t;
    }
    a
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}
