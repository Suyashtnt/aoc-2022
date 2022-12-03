use std::fs::read_to_string;

fn get_rucksack_compartment_items(rucksack: &str) -> (&str, &str) {
    rucksack.split_at(rucksack.len() / 2)
}

fn get_priority(character: char) -> u32 {
    let ascii_value = character as u8;

    // logic
    // if its bigger than a, its lowercase, else its uppercase
    Into::<u32>::into(if ascii_value >= b'a' {
        ascii_value - b'a'
    } else {
        ascii_value - b'A' + 26 // 26 is the offset for the uppercase letters
    }) + 1
}

fn main() {
    let input = read_to_string("./inputs/03.txt").unwrap();
    // part 1
    let part_1: u32 = input
        .lines()
        .map(|line| {
            let (left, right) = get_rucksack_compartment_items(line);

            let common_item = left.chars().find(|c| right.contains(*c)).unwrap();
            get_priority(common_item)
        })
        .sum();
    println!("Part 1: {}", part_1);

    // part 2
    let part_2: u32 = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|rucksack| {
            let (rucksack_1, rucksack_2, rucksack_3) = (rucksack[0], rucksack[1], rucksack[2]);
            let common_item = rucksack_1
                .chars()
                .find(|c| rucksack_2.contains(*c) && rucksack_3.contains(*c))
                .unwrap();

            get_priority(common_item)
        })
        .sum();

    println!("Part 2: {}", part_2);
}
