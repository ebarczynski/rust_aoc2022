use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let temp = input
        .lines()
        .flat_map(|s| s.split(&['-', ','][..]))
        .map(|x| x.parse::<u32>().unwrap())
        .tuples::<(_, _, _, _)>()
        .map(|t| t.2 >= t.0 && t.3 <= t.1 || t.0 >= t.2 && t.1 <= t.3)
        .filter(|v| *v)
        .count() as u32;
    temp.into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let temp = input
        .lines()
        .flat_map(|s| s.split(&['-', ','][..]))
        .map(|x| x.parse::<u32>().unwrap())
        .tuples::<(_, _, _, _)>()
        .map(|t| t.2 <= t.1 && t.3 >= t.0)
        .filter(|v| *v)
        .count() as u32;
    temp.into()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
