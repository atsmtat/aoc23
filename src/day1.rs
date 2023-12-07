pub fn part_one(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let first = line.chars().find_map(|c| c.to_digit(10)).unwrap();
        let last = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
        sum += (first * 10) + last;
    }
    sum
}

fn string_to_digit(input: &str) -> Option<u32> {
    if let Some(Some(dig)) = input.chars().next().map(|c| c.to_digit(10)) {
        return Some(dig);
    }

    for (word, dig) in [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ] {
        if input.starts_with(word) {
            return Some(dig);
        }
    }
    None
}

pub fn part_two(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let first = (0..line.len())
            .find_map(|i| string_to_digit(&line[i..]))
            .unwrap();
        let last = (0..line.len())
            .rev()
            .find_map(|i| string_to_digit(&line[i..]))
            .unwrap();
        sum += (first * 10) + last;
    }
    sum
}
