use std::io;

fn main() {
    let mut sum: i32 = 0;
    let mut is_valid: bool = true;

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        let tokens: Vec<&str> = input.split_whitespace().collect();

        if tokens.len() != 1 {
            is_valid = false;
            break;
        }

        match tokens[0].parse::<i32>() {
            Ok(-1) => break,
            Ok(num) => sum += num,
            Err(_) => {
                is_valid = false;
                break;
            }
        }
    }

    if is_valid {
        println!("{}", sum);
    } else {
        println!("NaN");
    }
}
