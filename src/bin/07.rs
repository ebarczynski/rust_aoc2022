use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut sizes_map = HashMap::new();
    let mut current_path = Vec::new();

    for line in input.lines() {
        match line.split_once(' ').unwrap() {
            ("$", "ls") => {}
            ("dir", _) => {}
            ("$", change_directory) => match change_directory.split_once(' ').unwrap() {
                ("cd", "..") => {
                    current_path.pop();
                }
                ("cd", catalog) => {
                    current_path.push(catalog);
                }
                _ => panic!(),
            },
            (size, _name) => {
                let size: u32 = size.parse().unwrap();
                for idx in 0..current_path.len() {
                    let path = current_path.get(..=idx).unwrap().to_owned();
                    *sizes_map.entry(path).or_insert(0) += size;
                }
            }
        };
    }

    Some(
        sizes_map
            .into_values()
            .filter(|&size| size <= 100000 as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sizes_map = HashMap::new();
    let mut current_path = Vec::new();

    for line in input.lines() {
        match line.split_once(' ').unwrap() {
            ("$", "ls") => {}
            ("dir", _) => {}
            ("$", change_directory) => match change_directory.split_once(' ').unwrap() {
                ("cd", "..") => {
                    current_path.pop();
                }
                ("cd", catalog) => {
                    current_path.push(catalog);
                }
                _ => panic!(),
            },
            (size, _name) => {
                let size: u32 = size.parse().unwrap();
                for idx in 0..current_path.len() {
                    let path = current_path.get(..=idx).unwrap().to_owned();
                    *sizes_map.entry(path).or_insert(0) += size;
                }
            }
        };
    }

    let total = 70000000;
    let needed = 30000000;
    let used = sizes_map.get(&["/"].to_vec()).unwrap();
    let free = total - used;

    // println!("total {:?}", total);
    // println!("needed {:?}", needed);
    // println!("free {:?}", free);
    // println!("{:?}", sizes_map);

    let min = sizes_map
        .into_values()
        .filter(|&size| free + size >= needed as u32)
        .min();
    println!("{:?}", min);
    min
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
