use std::{fs::read_to_string, ops::RangeInclusive, str::Lines};

fn to_assignment_range(assignment: &str) -> RangeInclusive<u32> {
    let assignment = assignment.split_once('-').unwrap();
    let start = assignment.0.parse().unwrap();
    let end = assignment.1.parse().unwrap();
    start..=end
}

fn parse_pair(line: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let pairs = line.split_once(',').unwrap();

    let pair1 = to_assignment_range(pairs.0);
    let pair2 = to_assignment_range(pairs.1);

    (pair1, pair2)
}

fn part_1(input: Lines) -> u32 {
    input
        .filter(|line| {
            let (pair1, pair2) = parse_pair(line);
            (pair1.contains(pair2.start()) && pair1.contains(pair2.end()))
                || (pair2.contains(pair1.start()) && pair2.contains(pair1.end()))
        })
        .count() as u32
}

fn part_2(input: Lines) -> u32 {
    input
        .filter(|line| {
            let (pair1, pair2) = parse_pair(line);
            pair1.contains(pair2.start())
                || pair1.contains(pair2.end())
                || pair2.contains(pair1.start())
                || pair2.contains(pair1.end())
        })
        .count() as u32
}

fn main() {
    let input = read_to_string("./inputs/04.txt").unwrap();
    let lines = input.lines();

    println!("Part 1: {}", part_1(lines.clone()));
    println!("Part 2: {}", part_2(lines));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2-4,6-8\n\
2-3,4-5\n\
5-7,7-9\n\
2-8,3-7\n\
6-6,4-6\n\
2-6,4-8";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT.lines()), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT.lines()), 4);
    }
}
