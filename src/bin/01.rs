use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u64> {
    let chunks: _ = input
        .lines()
        .map(|n| n.parse::<u64>().unwrap_or_default())
        .collect::<Vec<_>>()
        .split(|&e| e == u64::default())
        .map(|s| s.into())
        .collect::<Vec<Vec<_>>>();

    // println!("{:?}", chunks);

    let answer: _ = chunks.iter().map(|x| x.iter().sum()).max();
    answer
}

pub fn part_two(input: &str) -> Option<u64> {
    let chunks: _ = input
        .lines()
        .map(|n| n.parse::<u64>().unwrap_or_default())
        .collect::<Vec<_>>()
        .split(|&e| e == u64::default())
        .map(|s| s.into())
        .collect::<Vec<Vec<_>>>();

    let mut sorted: _ = chunks
        .iter()
        .map(|x| x.iter().sum())
        .collect::<Vec<u64>>()
        .into_iter()
        .sorted();

    let sum_of_3_biggest_numbers =
        sorted.next_back()? + sorted.next_back()? + sorted.next_back()?;

    Some(sum_of_3_biggest_numbers)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
