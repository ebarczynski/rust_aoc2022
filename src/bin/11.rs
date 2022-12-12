use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum Operand {
    Value(u128),
    Old,
}
#[derive(Debug, PartialEq)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug, PartialEq)]
struct Monkey {
    items: VecDeque<u128>,
    operation: Operation,
    operand: Operand,
    test_divisible_by: u32,
    test_true_monkey: u8,
    test_false_monkey: u8,
}

fn solve(rounds: usize, worry_factor: u64, mut monkeys: Vec<Monkey>) -> Option<u64> {
    let mut inspections = Vec::<u64>::new();
    for _monkey_index in 0..monkeys.len() {
        inspections.push(0);
    }
    let greatest_common_divisor = monkeys
        .iter()
        .map(|m| m.test_divisible_by as u64)
        .product::<u64>();
    for _round in 0..rounds {
        for monkey_index in 0..monkeys.len() {
            while let Some(item) = monkeys[monkey_index].items.pop_front() {
                inspections[monkey_index] += 1;

                let operand_value = match monkeys[monkey_index].operand {
                    Operand::Value(x) => x,
                    Operand::Old => item,
                };
                let new_value = match monkeys[monkey_index].operation {
                    Operation::Add => item + operand_value,
                    Operation::Multiply => item * operand_value,
                };
                let worry_level =
                    (new_value % greatest_common_divisor as u128) / worry_factor as u128;

                if worry_level % monkeys[monkey_index].test_divisible_by as u128 != 0 {
                    let false_index = monkeys[monkey_index].test_false_monkey;
                    monkeys[false_index as usize].items.push_back(worry_level);
                } else {
                    let true_index = monkeys[monkey_index].test_true_monkey;
                    monkeys[true_index as usize].items.push_back(worry_level);
                }
            }
        }
    }
    inspections.sort();
    println!("{:?}", inspections);
    Some(inspections.iter().rev().take(2).product())
}

fn prepare_monkey_data(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::<Monkey>::new();

    input.split("\n\n").for_each(|block| {
        let mut items = VecDeque::<u128>::new();
        let mut operation = Operation::Add;
        let mut operand = Operand::Old;
        let mut test_divisible_by = 0;
        let mut test_true_monkey = 0;
        let mut test_false_monkey = 0;

        for (i, line) in block.lines().enumerate() {
            match i {
                1 => {
                    line.split_once(": ")
                        .unwrap()
                        .1
                        .split(", ")
                        .for_each(|item| {
                            items.push_back(
                                item.parse::<u128>()
                                    .expect("Item list parsing failed for {item}"),
                            )
                        });
                }
                2 => {
                    operand = match line
                        .split_once("+ ")
                        .unwrap_or_else(|| {
                            operation = Operation::Multiply;
                            line.split_once("* ").unwrap()
                        })
                        .1
                        .parse::<u128>()
                    {
                        Ok(value) => Operand::Value(value),
                        Err(_) => Operand::Old,
                    }
                }
                3 => {
                    test_divisible_by = line
                        .split_once("by ")
                        .unwrap()
                        .1
                        .parse::<u32>()
                        .expect("test_divisible_by parsing failed for {line}")
                }
                4 => {
                    test_true_monkey = line
                        .split_once("monkey ")
                        .unwrap()
                        .1
                        .parse::<u8>()
                        .expect("test_true_monkey parsing failed for {line}")
                }
                5 => {
                    test_false_monkey = line
                        .split_once("monkey ")
                        .unwrap()
                        .1
                        .parse::<u8>()
                        .expect("test_true_monkey parsing failed for {line}")
                }
                _ => {}
            }
        }
        monkeys.push(Monkey {
            items,
            operation,
            operand,
            test_divisible_by,
            test_true_monkey,
            test_false_monkey,
        });
    });
    monkeys
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(20, 3, prepare_monkey_data(input))
}
pub fn part_two(input: &str) -> Option<u64> {
    solve(10000, 1, prepare_monkey_data(input))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
