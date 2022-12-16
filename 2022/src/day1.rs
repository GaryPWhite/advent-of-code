use std::fs;
use std::vec;

// read elves input and output how many total calories
// the elf holding the most calories is holding.
pub fn elvescalories() -> String {
    let raw = fs::read_to_string("./src/inputs/day1").expect("Could not read day 1");

    let elves = raw.split("\n\n").map(|s| {s.lines()});
    let most= elves.fold(vec![0,0,0], |mut acc, curr| {
        let calories = curr.fold(0, |acc, line| {
            acc + line.parse::<i32>().unwrap()
        });
        
        // check if calories is more than any part of the list
        
        if let Some(index) = acc.clone().into_iter().enumerate().find_map(|(index, caloriecount)| {
            if caloriecount < calories {
                return Some(index)
            }
            return None
        }) {
            acc.insert(index, calories);
            acc.truncate(3);
        }
        acc
    });

    let total = most.into_iter().fold(0, |total, calories| {
        total + calories
    });


    total.to_string()
}
