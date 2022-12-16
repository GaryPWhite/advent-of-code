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
    let raw = fs::read_to_string("./src/inputs/day3").expect("Could not read day 3");
    let mut rawlines = raw.lines();
    let mut groups = vec![];
    while let Some(rucksack1) = rawlines.next() {
        // make a group of 3 and move on
        let rucksack2 = rawlines.next().unwrap();
        let rucksack3 = rawlines.next().unwrap();
        groups.push((rucksack1, rucksack2, rucksack3))
    }
    for (rucksack1, rucksack2, rucksack3) in groups{
        let shared = shared_rucksack_char(rucksack1, rucksack2, rucksack3);
        total_priority = total_priority + priority_map.get(&shared).unwrap();
    }
    total_priority
}

fn shared_rucksack_char(rucksack1: &str, rucksack2: &str, rucksack3: &str) -> char {
    let mut map = HashSet::<char>::new();
    let mut map2 = HashSet::<char>::new();

    for c in rucksack1.chars() {
        map.insert(c);
    }
    for c in rucksack2.chars() {
        if map.contains(&c) {
            map2.insert(c); // shows up in both bags
        }
    }
    for c in rucksack3.chars() {
        if map2.contains(&c) {
            return c
        }
    }
    '!'
}
