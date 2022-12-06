use std::{collections::VecDeque, vec};

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let number_of_containers: usize = input
        .lines()
        .filter(|&x| x.starts_with(" 1"))
        .map(|x| x.split_whitespace().next_back().unwrap().parse().unwrap())
        .next_back()
        .unwrap();

    let instructions: Vec<(usize, usize, usize)> = input
        .lines()
        .filter(|&x| x.starts_with("move"))
        .flat_map(|s| {
            s.split(|c| char::is_alphabetic(c) || c == ' ')
                .filter_map(|x| x.parse().ok())
                .collect_tuple()
        })
        .collect_vec();

    let container_input = input
        .lines()
        .take(8)
        .flat_map(|s| {
            s.split_terminator(|c| c == '[' || c == ']')
                .flat_map(|s| {
                    let count = s.len() / 4;
                    let vec: Vec<&str> = vec!["0"; count];
                    if vec.len() >= 1 {
                        vec
                    } else {
                        vec![s]
                    }
                })
                .filter(|&s| !s.is_empty() && !s.eq(" "))
        })
        .collect_vec();

    println!("container_input {:?}", container_input);

    let mut containers: Vec<Vec<String>> = Vec::new();

    for _ in 0..number_of_containers {
        containers.push(Vec::new())
    }

    let mut j: usize = 0;
    for i in container_input {
        if j >= number_of_containers {
            j = 0;
        };

        if i != "0" {
            containers[j].push(i.to_string());
        }

        j += 1;
    }

    for i in 0..number_of_containers {
        containers[i].reverse()
    }

    println!("number of containers {:?}", number_of_containers);
    println!("containers: {:?}", containers);
    println!("{:?}", instructions);

    instructions.iter().for_each(|i| {
        for _ in 0..i.0 {
            let temp = containers[i.1 - 1].pop().unwrap();
            containers[i.2 - 1].push(temp);
        }
    });

    println!(
        "after {:?}",
        containers.iter().flat_map(|v| v.last()).collect_vec()
    );
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let number_of_containers: usize = input
        .lines()
        .filter(|&x| x.starts_with(" 1"))
        .map(|x| x.split_whitespace().next_back().unwrap().parse().unwrap())
        .next_back()
        .unwrap();

    let instructions: Vec<(usize, usize, usize)> = input
        .lines()
        .filter(|&x| x.starts_with("move"))
        .flat_map(|s| {
            s.split(|c| char::is_alphabetic(c) || c == ' ')
                .filter_map(|x| x.parse().ok())
                .collect_tuple()
        })
        .collect_vec();

    let container_input = input
        .lines()
        .take(8)
        .flat_map(|s| {
            s.split_terminator(|c| c == '[' || c == ']')
                .flat_map(|s| {
                    let count = s.len() / 4;
                    let vec: Vec<&str> = vec!["0"; count];
                    if vec.len() >= 1 {
                        vec
                    } else {
                        vec![s]
                    }
                })
                .filter(|&s| !s.is_empty() && !s.eq(" "))
        })
        .collect_vec();

    println!("container_input {:?}", container_input);

    let mut containers: Vec<VecDeque<String>> = Vec::new();

    for _ in 0..number_of_containers {
        containers.push(VecDeque::new())
    }

    let mut j: usize = 0;
    for i in container_input {
        if j >= number_of_containers {
            j = 0;
        };

        if i != "0" {
            containers[j].push_front(i.to_string());
        }

        j += 1;
    }

    println!("number of containers {:?}", number_of_containers);
    println!("containers: {:?}", containers);
    println!("{:?}", instructions);

    instructions.iter().for_each(|i| {
        let from_size = containers[i.1 - 1].len();
        let mut temp = containers[i.1 - 1]
            .drain((from_size - i.0)..from_size)
            .collect::<VecDeque<_>>();
        containers[i.2 - 1].append(&mut temp);
    });

    println!(
        "after {:?}",
        containers.iter().flat_map(|v| v.back()).collect_vec()
    );
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
