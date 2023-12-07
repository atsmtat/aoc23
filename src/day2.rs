use std::cmp;
use std::str::FromStr;

#[derive(Debug, Default)]
struct Draw {
    red: isize,
    green: isize,
    blue: isize,
}

impl Draw {
    fn is_valid(&self, max: &Draw) -> bool {
        self.red <= max.red && self.green <= max.green && self.blue <= max.blue
    }

    fn upper_bound(&self, other: &Draw) -> Self {
        Draw {
            red: cmp::max(self.red, other.red),
            green: cmp::max(self.green, other.green),
            blue: cmp::max(self.blue, other.blue),
        }
    }
}

#[derive(Debug)]
struct DrawParseErr;

impl FromStr for Draw {
    type Err = DrawParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut d = Draw {
            red: 0,
            green: 0,
            blue: 0,
        };
        for (count, color) in s.split(',').map(|col| col.trim().split_once(' ').unwrap()) {
            let count = count.parse::<isize>().unwrap();
            match color {
                "red" => d.red = count,
                "green" => d.green = count,
                "blue" => d.blue = count,
                _ => return Err(DrawParseErr),
            }
        }
        Ok(d)
    }
}

struct Game(usize);

#[derive(Debug)]
struct GameParseErr;

impl FromStr for Game {
    type Err = GameParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s
            .strip_prefix("Game ")
            .and_then(|s| s.parse::<usize>().ok())
            .ok_or(GameParseErr)?;
        Ok(Game(id))
    }
}

pub fn part_one(input: &str) -> usize {
    let mut sum = 0;
    let max_draw = Draw {
        red: 12,
        green: 13,
        blue: 14,
    };

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let (game, draws) = line.split_once(':').unwrap();
        let game = game.trim().parse::<Game>().unwrap();

        let invalid = draws
            .split(';')
            .filter(|draw| {
                let d = draw.trim().parse::<Draw>().unwrap();
                !d.is_valid(&max_draw)
            })
            .count();

        if invalid == 0 {
            sum += game.0;
        }
    }
    sum
}

pub fn part_two(input: &str) -> isize {
    let mut sum = 0;

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let (_, draws) = line.split_once(':').unwrap();

        let m = Draw::default();
        let res = draws.split(';').fold(m, |rm, draw| {
            let d = draw.trim().parse::<Draw>().unwrap();
            d.upper_bound(&rm)
        });
        sum += res.red * res.blue * res.green;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn test_puzzle_example() {
        let input = r"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
        assert_eq!(8, part_one(input));
        assert_eq!(2286, part_two(input));
    }
}
