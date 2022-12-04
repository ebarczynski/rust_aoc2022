use std::collections::HashMap;
use std::collections::HashSet;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let alphabet = (b'a'..b'z' + 1).chain(b'A'..b'Z' + 1).map(|i| i as char);
    let mapping: HashMap<_, _> = HashMap::from_iter(alphabet.zip(1u8..53u8));

    let priorities_sum: u32 = input
        .lines()
        .map(|s| {
            let pair = s.split_at(s.len() / 2);
            let hash_set: HashSet<char> = pair.0.chars().collect();
            pair.1.chars().find(|s| hash_set.contains(s)).unwrap()
        })
        .map(|c| mapping.get(&c).unwrap())
        .fold(0, |acc, &x| acc + u32::from(x));

    Some(priorities_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let alphabet = (b'a'..b'z' + 1).chain(b'A'..b'Z' + 1).map(|i| i as char);
    let mapping: HashMap<_, _> = HashMap::from_iter(alphabet.zip(1u8..53u8));

    let priorities_sum: _ = input
        .lines()
        .tuples::<(_, _, _)>()
        .map(|s| {
            let set0: HashSet<char> = s.0.chars().collect();
            let set1: HashSet<char> = s.1.chars().collect();
            let set2: HashSet<char> = s.2.chars().collect();
            let intersection_0_1: HashSet<_> = set0.intersection(&set1).map(|&c| c).collect();
            let shared_value: _ = set2.intersection(&intersection_0_1).take(1).next().unwrap();
            mapping.get(shared_value).unwrap()
        })
        .fold(0, |acc, &x| acc + u32::from(x));

    println!("{:?}", priorities_sum);
    Some(priorities_sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
