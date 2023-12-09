use std::ops;

type Range = ops::Range<u64>;

// Range and whether a mapping was found for it or not
struct MappedRange(Range, bool);

fn map_range(kr: &Range, mr_and_offset: (&Range, isize)) -> Vec<MappedRange> {
    let mut mapped = vec![];
    let (mr, offset) = mr_and_offset;

    let apply_offset = |r: Range| {
        let start = (r.start as isize + offset) as u64;
        let end = (r.end as isize + offset) as u64;
        MappedRange(start..end, true)
    };

    let one_to_one = |r: Range| MappedRange(r, false);

    if kr.end <= mr.start || mr.end <= kr.start {
        // disjoint ranges
        mapped.push(one_to_one(kr.clone()));
    } else {
        // intersecting ranges: [ks, ke) and [ms, me)
        if kr.start <= mr.start {
            // ks.....
            //    ms..
            if kr.start != mr.start {
                // [ks, ms) maps one-to-one
                mapped.push(one_to_one(kr.start..mr.start));
            }

            if kr.end <= mr.end {
                // ks......ke
                //    ms...me
                mapped.push(apply_offset(mr.start..kr.end));
                // ke..me is ignored as it's out of key's range
            } else {
                // ks..........ke
                //    ms.....me
                mapped.push(apply_offset(mr.start..mr.end));
                // [me, ke) maps one-to-one
                mapped.push(one_to_one(mr.end..kr.end));
            }
        } else {
            //    ks....
            // ms.......
            if kr.end <= mr.end {
                //    ks....ke
                // ms.......me
                mapped.push(apply_offset(kr.start..kr.end));
                // ke..me is out of key's range
            } else {
                //    ks.......ke
                // ms......me
                mapped.push(apply_offset(kr.start..mr.end));
                // [me, ke) maps one-to-one
                mapped.push(one_to_one(mr.end..kr.end));
            }
        }
    }
    mapped
}

// Apply a map (e.g. seed-to-soil map) to a given set of key
// ranges. lines is expected to point to the first entry in the map.
fn apply_map<'a, I>(mut key_ranges: Vec<Range>, lines: &mut I) -> Vec<Range>
where
    I: Iterator<Item = &'a str>,
{
    let map_ranges: Vec<_> = lines
        .take_while(|l| !l.trim().is_empty())
        .map(|l| {
            let nums: Vec<u64> = l
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect();
            let offset = nums[0] as isize - nums[1] as isize;
            let map_range = nums[1]..nums[1] + nums[2];
            (map_range, offset)
        })
        .collect();

    let mut mapped = vec![];

    for (mr, offset) in map_ranges {
        let mut next_keys = vec![];

        for kr in key_ranges {
            let output = map_range(&kr, (&mr, offset));
            for mapped_range in output {
                if mapped_range.1 {
                    // mapping found
                    mapped.push(mapped_range.0);
                } else {
                    // mapping not found, so try for next map range
                    next_keys.push(mapped_range.0);
                }
            }
        }

        key_ranges = next_keys;
    }

    // key-ranges whose mapping was not found are considered
    // one-to-one mapped, so push them as it is into mapped vector.
    mapped.extend(key_ranges);
    mapped
}

pub fn part_one(input: &str) -> u64 {
    let mut lines = input.lines();

    let (_, seeds) = lines
        .by_ref()
        .find(|l| !l.trim().is_empty())
        .unwrap()
        .split_once(':')
        .unwrap();

    let seeds: Vec<u64> = seeds
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    let mut ranges: Vec<Range> = seeds.iter().map(|s| *s..(*s + 1)).collect();

    lines.by_ref().next();
    while let Some(_) = lines.by_ref().next() {
        ranges = apply_map(ranges, lines.by_ref());
    }
    ranges.iter().map(|r| r.start).min().unwrap()
}

pub fn part_two(input: &str) -> u64 {
    let mut lines = input.lines();

    let (_, seeds) = lines
        .by_ref()
        .find(|l| !l.trim().is_empty())
        .unwrap()
        .split_once(':')
        .unwrap();

    let seeds: Vec<u64> = seeds
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    let mut ranges: Vec<Range> = (0..seeds.len())
        .step_by(2)
        .map(|i| {
            let start = seeds[i];
            let delta = seeds[i + 1];
            start..start + delta + 1
        })
        .collect();

    lines.by_ref().next();
    while let Some(_) = lines.by_ref().next() {
        ranges = apply_map(ranges, lines.by_ref());
    }
    ranges.iter().map(|r| r.start).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn test_puzzle_example() {
        let input = r"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        assert_eq!(35, part_one(input));
        assert_eq!(46, part_two(input));
    }
}
