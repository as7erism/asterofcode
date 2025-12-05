pub struct Solution;

impl crate::Solution for Solution {
    fn part_one(input: &str) {
        let mut ranges = Vec::new();

        let mut lines = input.lines();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            let range = line
                .split('-')
                .map(|id| id.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            ranges.push((range[0], range[1]));
        }

        println!(
            "{}",
            lines
                .map(|id| id.parse::<u64>().unwrap())
                .filter(|id| ranges.iter().any(|(begin, end)| id >= begin && id <= end))
                .count()
        );
    }

    fn part_two(_input: &str) {
        todo!();
    }
}
