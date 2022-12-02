// 1 for A X Rock
// 2 for B Y Paper
// 3 for C Z Scissors
pub fn part_one(input: &str) -> Option<i32> {
    let itermediate: _ = input
        .lines()
        .map(|n| match n {
            "A Y" => 8,
            "A X" => 4,
            "A Z" => 3,
            "B Y" => 5,
            "B X" => 1,
            "B Z" => 9,
            "C Y" => 2,
            "C X" => 7,
            "C Z" => 6,
            &_ => todo!(),
        })
        .fold(0, |acc, x| acc + x);

    // println!("{:?}", itermediate);

    Some(itermediate)
}

pub fn part_two(input: &str) -> Option<i32> {
    let itermediate: _ = input
        .lines()
        .map(|n| match n {
            "A Y" => 3 + 1,
            "A X" => 0 + 3,
            "A Z" => 6 + 2,
            "B Y" => 3 + 2,
            "B X" => 0 + 1,
            "B Z" => 6 + 3,
            "C Y" => 3 + 3,
            "C X" => 0 + 2,
            "C Z" => 6 + 1,
            &_ => todo!(),
        })
        .fold(0, |acc, x| acc + x);
    println!("{:?}", itermediate);

    Some(itermediate)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
