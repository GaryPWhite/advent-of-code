mod day1;
mod day2;
mod day3;

fn main() {
    println!("Day one: The top 3 elves are carrying {} calories", day1::elvescalories());
    println!("Day two: The score from the strategy sheet is {}", day2::rps());
    println!("Day three: priority sum is: {}", day3::rucksackpriority());
}

