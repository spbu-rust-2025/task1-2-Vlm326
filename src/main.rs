use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut sum = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let trimmed = line.trim();

        if trimmed == "-1" {
            break;
        }

        match trimmed.parse::<i32>() {
            Ok(n) if n > 0 => sum += n,
            _ => {
                println!("NaN");
                return;
            }
        }
    }

    println!("{}", sum);
}
