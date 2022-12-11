use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let terrain_map = input
        .lines()
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let mut visible = terrain_map
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(line_i, _)| {
                    i == 0 || i == terrain_map.len() - 1 || line_i == 0 || line_i == line.len() - 1
                })
                .collect_vec()
        })
        .collect_vec();

    for y in 0..terrain_map.len() {
        let mut current_tree = 0;
        for x in 0..terrain_map[0].len() {
            if x == 0 {
                current_tree = terrain_map[y][x];
            } else if terrain_map[y][x] > current_tree {
                current_tree = terrain_map[y][x];
                visible[y][x] = true;
            }
        }
    }

    for y in (0..terrain_map.len()).rev() {
        let mut current_tree = 0;
        for x in (0..terrain_map[0].len()).rev() {
            if x == terrain_map[0].len() - 1 {
                current_tree = terrain_map[y][x];
            } else if terrain_map[y][x] > current_tree {
                current_tree = terrain_map[y][x];
                visible[y][x] = true;
            }
        }
    }

    for y in (0..terrain_map.len()).rev() {
        let mut current_tree = 0;
        for x in (0..terrain_map[0].len()).rev() {
            if x == terrain_map[0].len() - 1 {
                current_tree = terrain_map[y][x];
            } else if terrain_map[y][x] > current_tree {
                current_tree = terrain_map[y][x];
                visible[y][x] = true;
            }
        }
    }
    for x in 0..terrain_map[0].len() {
        let mut current_tree = 0;
        for y in 0..terrain_map.len() {
            if y == terrain_map[0].len() - 1 {
                current_tree = terrain_map[y][x];
            } else if terrain_map[y][x] > current_tree {
                current_tree = terrain_map[y][x];
                visible[y][x] = true;
            }
        }
    }

    for x in (0..terrain_map[0].len()).rev() {
        let mut current_tree = 0;
        for y in (0..terrain_map.len()).rev() {
            if y == terrain_map[0].len() - 1 {
                current_tree = terrain_map[y][x];
            } else if terrain_map[y][x] > current_tree {
                current_tree = terrain_map[y][x];
                visible[y][x] = true;
            }
        }
    }

    // println!("{:?}", visible);
    Some(
        visible
            .iter()
            .flatten()
            .fold(0, |acc, &x| acc + u32::from(x)),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let terrain_map = input
        .lines()
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let mut scores_map = terrain_map
        .iter()
        .map(|line| line.iter().map(|_| 0).collect_vec())
        .collect_vec();

    let y_max = terrain_map.len();
    let x_max = terrain_map[0].len();

    //source: https://youtu.be/59CEnP-5eRg
    for (y_index, tree_line) in terrain_map.iter().enumerate() {
        for (x_index, treehouse_height) in tree_line.iter().enumerate() {
            let mut scores = [0, 0, 0, 0];

            // to left
            for x_position in (0..x_index).rev() {
                if terrain_map[y_index][x_position] < *treehouse_height {
                    scores[0] += 1;
                } else {
                    scores[0] += 1;
                    break;
                }
            }
            // to right
            for x_position in (x_index + 1)..x_max {
                if terrain_map[y_index][x_position] < *treehouse_height {
                    scores[1] += 1;
                } else {
                    scores[1] += 1;
                    break;
                }
            }

            // to up
            for y_position in (0..y_index).rev() {
                if terrain_map[y_position][x_index] < *treehouse_height {
                    scores[2] += 1;
                } else {
                    scores[2] += 1;
                    break;
                }
            }
            // to down
            for y_position in (y_index + 1)..y_max {
                if terrain_map[y_position][x_index] < *treehouse_height {
                    scores[3] += 1;
                } else {
                    scores[3] += 1;
                    break;
                }
            }
            let scenic_score: u32 = scores.iter().product();

            scores_map[y_index][x_index] = scenic_score;
        }
    }

    // println!("{:?}", scores_map);
    scores_map.iter().flatten().max().copied()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
