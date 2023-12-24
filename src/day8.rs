use num::integer::lcm;

use std::collections::HashMap;

type Map = HashMap<String, (String, String)>;

struct Network {
    map: Map,
    seq: String,
}

impl Network {
    fn new(input: &str) -> Self {
        let mut lines = input.lines().filter(|l| !l.trim().is_empty());
        let seq = lines.next().unwrap().to_string();

        let mut map = HashMap::new();

        for line in lines {
            let (node, adj) = line
                .split_once('=')
                .map(|(l, r)| (l.trim(), r.trim()))
                .unwrap();

            let (left, right) = adj
                .split_once(',')
                .map(|(l, r)| {
                    (
                        l.strip_prefix('(').unwrap().trim(),
                        r.strip_suffix(')').unwrap().trim(),
                    )
                })
                .unwrap();

            map.insert(node.to_string(), (left.to_string(), right.to_string()));
        }

        Network { map, seq }
    }

    fn iter_from<'a>(&'a self, start: &'a str) -> NetworkIter {
        NetworkIter::new(self, start)
    }
}

struct NetworkIter<'a> {
    network: &'a Network,
    curr: &'a str,
    steps: usize,
    seq_iter: std::iter::Cycle<std::str::Chars<'a>>,
}

impl<'a> NetworkIter<'a> {
    fn new(network: &'a Network, start: &'a str) -> Self {
        NetworkIter {
            network,
            curr: start,
            steps: 0,
            seq_iter: network.seq.chars().cycle(),
        }
    }

    fn steps(&self) -> usize {
        self.steps
    }
}

impl<'a> Iterator for NetworkIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.seq_iter.next().map(|cmd| {
            let (left, right) = self.network.map.get(self.curr).unwrap();
            self.curr = if cmd == 'R' { right } else { left };
            self.steps += 1;
            self.curr
        })
    }
}

pub fn part_one(input: &str) -> u64 {
    let nw = Network::new(input);
    let mut iter = nw.iter_from("AAA");
    for node in &mut iter {
        if node == "ZZZ" {
            return iter.steps() as u64;
        }
    }
    0
}

pub fn part_two(input: &str) -> u64 {
    let nw = Network::new(input);

    let mut walkers: Vec<_> = nw
        .map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| nw.iter_from(k))
        .collect();

    let steps: Vec<_> = walkers
        .iter_mut()
        .map(|w| {
            w.find(|n| n.ends_with('Z'));
            w.steps()
        })
        .collect();

    let res = steps.iter().fold(1, |m, s| lcm(m, *s));
    res as u64
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn test_puzzle_example_1() {
        let input = r"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
        assert_eq!(2, part_one(input));
    }

    #[test]
    fn test_puzzle_example_2() {
        let input = r"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
        assert_eq!(6, part_one(input));
    }

    #[test]
    fn test_puzzle_part_2() {
        let input = r"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
        assert_eq!(6, part_two(input));
    }
}
