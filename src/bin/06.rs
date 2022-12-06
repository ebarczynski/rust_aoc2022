use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let collection = input.trim_end();
    let window = 4;

    for i in window..collection.len() {
        let unique: HashSet<_> = HashSet::from_iter(collection[(i - window)..i].chars());
        if unique.len() == window {
            return Some(i as u32);
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let collection = input.trim_end();
    let window = 14;

    for i in window..collection.len() {
        let unique: HashSet<_> = HashSet::from_iter(collection[(i - window)..i].chars());
        if unique.len() == window {
            return Some(i as u32);
        }
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(10));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(29));
    }
}
