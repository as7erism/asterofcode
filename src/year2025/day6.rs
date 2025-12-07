use core::panic;

pub struct Solution;

impl crate::Solution for Solution {
    type OutputOne = u64;

    fn part_one(input: &str) -> Self::OutputOne {
        let mut lines = input.lines();
        let mut columns = Vec::from_iter(
            lines
                .next()
                .unwrap()
                .split_whitespace()
                .map(|operand| vec![operand.parse::<u64>().unwrap()]),
        );

        for line in lines {
            if line.starts_with('*') || line.starts_with('+') {
                return line.split_whitespace().zip(columns.into_iter()).fold(
                    0,
                    |acc, (operator, operands)| {
                        if operator == "*" {
                            acc + operands.iter().fold(1, |acc2, &operand| acc2 * operand)
                        } else {
                            acc + operands.iter().fold(0, |acc2, &operand| acc2 + operand)
                        }
                    },
                );
            }

            line.split_whitespace()
                .enumerate()
                .for_each(|(i, operand)| columns[i].push(operand.parse::<u64>().unwrap()));
        }

        panic!("input shouldve had an operator line!");
    }

    fn part_two(input: &str) -> Self::OutputTwo {
        let mut grid = Vec::<Vec<char>>::new();
        let mut operand_groups = vec![vec![]];

        for line in input.lines() {
            if line.starts_with('*') || line.starts_with('+') {
                for col in 0..grid[0].len() {
                    if let Ok(operand) = grid
                        .iter()
                        .map(|row| row[col])
                        .filter(|digit| !digit.is_whitespace())
                        .collect::<String>()
                        .parse::<u64>()
                    {
                        operand_groups.last_mut().unwrap().push(operand);
                    } else {
                        operand_groups.push(Vec::new());
                    }
                }

                // we have one more operand group (the last one is empty) than operators now, but
                // `.zip()` ends after the first zipped iterator returns None, so its okay
                return line
                    .split_whitespace()
                    .zip(operand_groups.into_iter())
                    .fold(0, |acc, (operator, operands)| {
                        if operator == "*" {
                            acc + operands.iter().fold(1, |acc2, &operand| acc2 * operand)
                        } else {
                            acc + operands.iter().fold(0, |acc2, &operand| acc2 + operand)
                        }
                    });
            }

            grid.push(Vec::from_iter(line.chars()));
        }

        panic!("input shouldve had an operator line!");
    }
}
