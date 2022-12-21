mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    println!("Day one: The top 3 elves are carrying {} calories", day1::elvescalories());
    println!("Day two: The score from the strategy sheet is {}", day2::rps());
    println!("Day three: priority sum is: {}", day3::rucksackpriority());
    println!("Day four: {} sections overlap", day4::section_overlap());
    println!("Day five: crates are: {} ", day5::crates());
    println!("Day six: packet info: {}", day6::packet());
}

