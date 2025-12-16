pub struct Solution;

impl crate::Solution for Solution {
    type OutputOne = u64;

    fn part_one(input: &str) -> Self::OutputOne {
        let points = input
            .lines()
            .map(|l| {
                let mut components = l.split(',');
                let x = components.next().unwrap().parse::<u64>().unwrap();
                let y = components.next().unwrap().parse::<u64>().unwrap();
                (x, y)
            })
            .collect::<Vec<_>>();
        let mut max_area = 0;

        for i in 0..(points.len() - 1) {
            for j in (i + 1)..points.len() {
                let area = (points[i].0.abs_diff(points[j].0) + 1)
                    * (points[i].1.abs_diff(points[j].1) + 1);
                if area > max_area {
                    max_area = area;
                }
            }
        }

        max_area
    }

    fn part_two(_input: &str) -> Self::OutputTwo {
        todo!();
    }
}
