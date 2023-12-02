use itertools::Itertools;

pub fn run(input: &str) -> (usize, u32) {
    let (red, green, blue) = (12, 13, 14);
    let mut part1 = 0;
    let mut part2 = 0;
    for (id, line) in input.lines().enumerate() {
        let line = line.split_once(':').unwrap().1;
        let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);
        let words = line
            .split([';', ',', ' '])
            .filter(|x| !x.is_empty())
            .tuples();
        let draws = words
            .map(|(count, color)| {
                let count = count.parse::<u32>().unwrap();
                match color {
                    "red" => {
                        min_red = min_red.max(count);
                        count <= red
                    }
                    "green" => {
                        min_green = min_green.max(count);
                        count <= green
                    }
                    "blue" => {
                        min_blue = min_blue.max(count);
                        count <= blue
                    }
                    _ => panic!(),
                }
            })
            .collect_vec();
        if draws.iter().all(|x| *x) {
            part1 += id + 1;
        }
        part2 += min_red * min_green * min_blue;
    }
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn sample() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(run(input), (8, 2286));
    }
}
