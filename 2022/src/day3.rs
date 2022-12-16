use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn rucksackpriority() -> i32 {
    let mut priority_map = HashMap::<char, i32>::with_capacity(52);
    let mut priorityct = 1;

    for char in 'a'..='z' {
        priority_map.insert(char, priorityct);
        priorityct = priorityct + 1;
    }
    for char in 'A'..='Z' {
        priority_map.insert(char, priorityct);
        priorityct = priorityct + 1;
    }

    let mut total_priority = 0;
    let raw = fs::read_to_string("./src/inputs/day3").expect("Could not read day 1");
    for line in raw.lines() {
        let shared = shared_rucksack_char(line);
        total_priority = total_priority + priority_map.get(&shared).unwrap();
    }
    total_priority
}

fn shared_rucksack_char(rucksack: &str) -> char {
    let mut set = HashSet::<char>::new();

    let (first_half, second_half) = rucksack.split_at(((rucksack.len() / 2)) as usize);
    for char in first_half.chars() {
        set.replace(char);
    }
    for char in second_half.chars() {
        if set.contains(&char) {
            return char;
        }
    }
    '!'
}
