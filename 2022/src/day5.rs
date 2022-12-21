use std::fs;

pub fn crates() -> String {
    let mut crate_spots = read_vecs();
    let moves_vec = read_moves();

    for (ct, from, to) in moves_vec {
        // for _ in 0..ct { part 1
        //     let to_push = crate_spots.get_mut((from-1) as usize).unwrap().pop();
        //     crate_spots
        //         .get_mut((to-1) as usize)
        //         .unwrap()
        //         .push(to_push.unwrap())
        // }
        let mut buffer: Vec<char> = Vec::with_capacity(ct as usize);
        for _ in 0..ct {
            buffer.push(
                crate_spots
                    .get_mut((from - 1) as usize)
                    .unwrap()
                    .pop()
                    .unwrap(),
            );
        }
        for _ in 0..ct {
            crate_spots
                .get_mut((to - 1) as usize)
                .unwrap()
                .push(buffer.pop().unwrap());
        }
    }

    format!(
        "{}",
        crate_spots
            .iter()
            .map(|spot| { spot.last().unwrap() })
            .collect::<String>()
    )
}

fn read_vecs() -> Vec<Vec<char>> {
    let raw_crates_start =
        fs::read_to_string("./src/inputs/day5.init").expect("could not read day 5 input");

    let mut to_ret = Vec::<Vec<char>>::with_capacity(9);
    for _ in 1..=9 {
        to_ret.push(Vec::<char>::new())
    }

    let alphabet = 'A'..='Z';
    for line in raw_crates_start.lines().rev() {
        for (idx, c) in line.chars().skip(1).step_by(4).enumerate() {
            if alphabet.contains(&c) {
                let v = to_ret.get_mut(idx).unwrap();
                v.push(c);
            }
        }
    }

    to_ret
}

fn read_moves() -> Vec<(i32, i32, i32)> {
    let raw_moves = fs::read_to_string("./src/inputs/day5").expect("Could not read day 5 moves");

    raw_moves
        .lines()
        .map(|line| {
            let move_vec: Vec<i32> = line
                .split_whitespace()
                .skip(1)
                .step_by(2)
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            (move_vec[0], move_vec[1], move_vec[2])
            // how many, from, to
        })
        .collect()
}
