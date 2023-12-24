pub fn part_one(input: &str) -> usize {
    let mut lines = input.lines().filter(|l| !l.trim().is_empty()).map(|l| {
        let (_, nums) = l.split_once(':').unwrap();
        nums.split_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    });

    let time = lines.next().unwrap();
    let distance = lines.next().unwrap();

    time.iter()
        .zip(distance.iter())
        .map(|(t, d)| (0..*t).filter(|x| (t - x) * x > *d).count())
        .product()
}

pub fn part_two(input: &str) -> u64 {
    let mut lines = input.lines().filter(|l| !l.trim().is_empty()).map(|l| {
        let (_, nums) = l.split_once(':').unwrap();
        let nums = nums
            .split_whitespace()
            .flat_map(|num| num.chars())
            .collect::<String>();
        nums.parse::<u64>().unwrap()
    });

    let time = lines.next().unwrap();
    let distance = lines.next().unwrap();

    let binary_search = |mut lo: u64, mut up: u64| {
        while lo < up - 1 {
            let mid = lo + (up - lo) / 2;
            if (time - mid) * mid > distance {
                up = mid;
            } else {
                lo = mid;
            }
        }
        up
    };
    let lower = binary_search(0, time / 2);
    let ways = (time / 2 - lower + 1) * 2;

    if time % 2 == 0 {
        ways - 1
    } else {
        ways
    }
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn test_puzzle_example() {
        let input = r"
Time:      7  15   30
Distance:  9  40  200
";
        assert_eq!(288, part_one(input));
        assert_eq!(71503, part_two(input));
    }
}
