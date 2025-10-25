use std::io;

fn main() {
    let mut sum = 0;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "-1" {
            break;
        }

        let number: u32 = match input.parse() {
            Ok(n) if n > 0 => n,
            _ => {
                println!("NaN");
                return;
            }
        };

        sum += number;
    }

    println!("{}", sum);
}
