use std::fs;

pub fn section_overlap() -> i32 {
    let raw = fs::read_to_string("./src/inputs/day4").expect("Could not read day 4");

    raw.lines().fold(0, |acc, line| {
        // get pairs as tuples
        let (section1, section2) = parse_pairs(line);
        let (section1_low, section1_high) = section1;
        let (section2_low, section2_high) = section2;

        if (section1_high < section2_low) || (section1_low > section2_high)
        // no overlap
        {
            return acc;
        } else {
            return acc + 1;
        }
    })
}

fn parse_pairs(line: &str) -> ((i32, i32), (i32, i32)) {
    let mut sections: Vec<(i32, i32)> = line
        .split(",")
        .map(|x| {
            let mut section_numbers = x.split("-").map(|y| y.parse().unwrap());
            let section1 = section_numbers.next().unwrap();
            let section2 = section_numbers.next().unwrap();
            ((section1), (section2))
        })
        .collect();
    let section2 = sections.pop().unwrap();
    let section1 = sections.pop().unwrap();
    ((section1), (section2))
}
