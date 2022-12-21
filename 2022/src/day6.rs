use std::fs;

pub fn packet() -> String {
    let raw_signal = fs::read_to_string("./src/inputs/day6").expect("problems reading day 6");
    let mut buf: String = String::new();

    for (idx, char) in raw_signal.chars().enumerate() {
        
        if let Some(charidx) = buf.find(char) {
            for _ in 0..=charidx {
                buf.remove(0);
            }
        } else if buf.len() == 13 {
            return (idx+1).to_string();
        }
        if buf.len() > 13 {
            buf.remove(0);
        }
        buf.push(char);
               
    }

    (-1).to_string()
}