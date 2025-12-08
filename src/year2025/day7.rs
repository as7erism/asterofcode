use std::collections::HashMap;

pub struct Solution;

impl crate::Solution for Solution {
    type OutputOne = u32;
    type OutputTwo = u64;

    fn part_one(input: &str) -> Self::OutputOne {
        let mut lines = input.lines();
        let mut beams = lines
            .next()
            .unwrap()
            .chars()
            .map(|ch| ch == 'S')
            .collect::<Vec<_>>();
        let mut splits = 0;

        for c in lines
            .enumerate()
            .filter(|(line_num, _)| line_num & 1 == 1)
            .flat_map(|(_, line)| line.chars().enumerate().filter(|&(_, ch)| ch == '^'))
            .map(|(c, _)| c)
        {
            if beams[c] {
                beams[c] = false;
                splits += 1;

                if let Some(i) = c.checked_sub(1) {
                    beams[i] = true;
                }

                if let Some(beam) = beams.get_mut(c + 1) {
                    *beam = true;
                }
            }
        }

        splits
    }

    fn part_two(input: &str) -> Self::OutputTwo {
        let mut lines = input.lines();
        // this could be a vector but im too tired for all that bounds checking
        let mut acc_map = HashMap::<(usize, usize), u64>::new();
        let beam = lines
            .next()
            .unwrap()
            .chars()
            .enumerate()
            .find(|&(_, ch)| ch == 'S')
            .map(|(i, _)| i)
            .unwrap();

        let grid = lines
            .enumerate()
            .filter(|(line_num, _)| line_num & 1 == 1)
            .map(|(_, line)| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        for (r, row) in grid.iter().enumerate().rev() {
            for (c, &ch) in row.iter().enumerate() {
                let acc = if ch == '^' {
                    acc_map.get(&(r + 1, c - 1)).cloned().unwrap_or(1)
                        + acc_map.get(&(r + 1, c + 1)).cloned().unwrap_or(1)
                } else {
                    acc_map.get(&(r + 1, c)).cloned().unwrap_or(1)
                };

                acc_map.insert((r, c), acc);
            }
        }

        acc_map.get(&(0, beam)).cloned().unwrap()
    }
}
