use std::io;

fn main() {
    let mut sum = 0;
    let mut invalid_input = false;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "-1" {
            break;
        }

        match input.parse::<u32>() {
            Ok(n) if n > 0 => sum += n,
            _ => invalid_input = true,
        }
    }

    if invalid_input {
        println!("NaN");
    } else {
        println!("{}", sum);
    }
}
