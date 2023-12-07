use std::collections::{HashMap, HashSet};
use std::ops;

struct Schematic<'a> {
    map: Vec<&'a str>,
    num_row: isize,
    num_col: isize,
}

type GearAdjList = HashMap<(usize, usize), Vec<u64>>;

impl<'a> Schematic<'a> {
    fn new(input: &'a str) -> Self {
        let map: Vec<_> = input.lines().filter(|l| !l.is_empty()).collect();
        let num_row = map.len() as isize;
        let num_col = map[0].len() as isize;
        Schematic {
            map,
            num_row,
            num_col,
        }
    }

    fn adj_to_symbol(&self, row: usize, col: usize) -> bool {
        let irow = row as isize;
        let icol = col as isize;
        let row_range = 0..self.num_row;
        let col_range = 0..self.num_col;

        for dr in [-1, 0, 1] {
            for dc in [-1, 0, 1] {
                let (nrow, ncol) = (irow + dr, icol + dc);
                if row_range.contains(&nrow) && col_range.contains(&ncol) {
                    let c = self.map[nrow as usize].chars().nth(ncol as usize).unwrap();
                    if !c.is_ascii_digit() && c != '.' {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn get_adj_gears(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let irow = row as isize;
        let icol = col as isize;
        let row_range = 0..self.num_row;
        let col_range = 0..self.num_col;

        let mut gears = Vec::new();
        for dr in [-1, 0, 1] {
            for dc in [-1, 0, 1] {
                let (nrow, ncol) = (irow + dr, icol + dc);
                if row_range.contains(&nrow) && col_range.contains(&ncol) {
                    let c = self.map[nrow as usize].chars().nth(ncol as usize).unwrap();
                    if c == '*' {
                        gears.push((nrow as usize, ncol as usize));
                    }
                }
            }
        }
        gears
    }

    fn mark_adj_gears(
        &self,
        row: usize,
        col_range: ops::Range<usize>,
        num: u64,
        gear_adj: &mut GearAdjList,
    ) {
        let mut gears = HashSet::new();
        for col in col_range {
            gears.extend(self.get_adj_gears(row, col));
        }

        for gear in gears {
            gear_adj
                .entry(gear)
                .and_modify(|adj| {
                    adj.push(num);
                })
                .or_insert(vec![num]);
        }
    }
}

pub fn part_one(input: &str) -> u64 {
    let schem = Schematic::new(input);
    let mut sum = 0;

    for (ri, row) in schem.map.iter().enumerate() {
        let mut digits = row
            .char_indices()
            .filter(|(_, c)| c.is_ascii_digit())
            .peekable();

        while let Some((ci, _)) = digits.next() {
            let start = ci;

            let mut end = ci;
            while digits.next_if(|(nci, _)| *nci == end + 1).is_some() {
                end += 1;
            }
            end += 1;

            if (start..end).any(|ci| schem.adj_to_symbol(ri, ci)) {
                let dig_str = &row[start..end];
                sum += dig_str.parse::<u64>().unwrap();
            }
        }
    }
    sum
}

pub fn part_two(input: &str) -> u64 {
    let schem = Schematic::new(input);
    let mut gear_adj = GearAdjList::new();

    for (ri, row) in schem.map.iter().enumerate() {
        let mut digits = row
            .char_indices()
            .filter(|(_, c)| c.is_ascii_digit())
            .peekable();

        while let Some((ci, _)) = digits.next() {
            let start = ci;

            let mut end = ci;
            while digits.next_if(|(nci, _)| *nci == end + 1).is_some() {
                end += 1;
            }
            end += 1;
            let num = row[start..end].parse::<u64>().unwrap();
            schem.mark_adj_gears(ri, start..end, num, &mut gear_adj);
        }
    }

    gear_adj.values().fold(0, |sum, adj_nums| {
        if adj_nums.len() == 2 {
            let ratio = adj_nums[0] * adj_nums[1];
            sum + ratio
        } else {
            sum
        }
    })
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn test_puzzle_example() {
        let input = r"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        assert_eq!(4361, part_one(input));
        assert_eq!(467835, part_two(input));
    }
}
