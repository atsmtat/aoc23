use std::collections::HashSet;

fn collect_matches(input: &str) -> Vec<u64> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let (_, numbers) = line.split_once(':').unwrap();

            let (winning_nums, my_nums) = numbers.trim().split_once('|').unwrap();

            let winning_nums: HashSet<_> = winning_nums
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect();

            my_nums
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .filter(|num| winning_nums.contains(num))
                .count() as u64
        })
        .collect()
}

pub fn part_one(input: &str) -> u64 {
    let mut sum = 0;
    for matches in collect_matches(input) {
        if matches > 0 {
            sum += 2u64.pow(matches as u32 - 1);
        }
    }
    sum
}

pub fn part_two(input: &str) -> u64 {
    let matches = collect_matches(input);
    let mut dp_table = vec![0; matches.len()];

    for dpi in (0..matches.len()).rev() {
        let wins = matches[dpi];
        let accu_wins: u64 = (1..=wins).map(|i| dp_table[dpi + i as usize]).sum();
        // card itself + accumulated wins
        dp_table[dpi] = 1 + accu_wins;
    }
    dp_table.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn test_puzzle_example() {
        let input = r"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        assert_eq!(13, part_one(input));
        assert_eq!(30, part_two(input));
    }
}
