use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./inputs/01.txt").unwrap();
    // split all the elfs
    let elfs = input.split("\n\n");

    // for each elf, get its whole cal count
    let mut cal_count_sorted = elfs
        .map(|elf| {
            elf.lines()
                .map(|line| line.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<_>>();

    cal_count_sorted.sort();

    println!("Part 1: {}", cal_count_sorted.last().unwrap());
    println!(
        "Part 2: {}",
        &cal_count_sorted[cal_count_sorted.len() - 3..cal_count_sorted.len()]
            .iter()
            .sum::<usize>()
    );
}
