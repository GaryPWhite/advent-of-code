use std::fs;

pub fn rps() -> String {
    let raw = fs::read_to_string("./src/inputs/day2").expect("Could not read day 1");

    // makes the second match must easier
    let score_array = [3,1,2];

    let total = raw.lines().fold(0, |mut acc, line| {
        let w_l: i32 = match line.chars().last().unwrap() {
            'X'=>-1,
            'Y'=>0,
            'Z'=>1,
            _=>-1000000
        };
        acc = acc + ((w_l+1) * 3);

        acc = acc + match line.chars().nth(0).unwrap() {
            'A'=>score_array[((1+w_l)%3) as usize],
            'B'=>score_array[((2+w_l)%3) as usize],
            'C'=>score_array[((3+w_l)%3) as usize],
            _=>-1000000, // signal something is wrong with massively negative value
        };

        acc
    });


    total.to_string()
}
