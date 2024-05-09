use std::io;

fn main() {
    loop {
        println!("Please input your age in years!");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Could not read age. Make you an integer is used.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let age: u32 = to_days(guess);

        println!("You are roughly {age} days old!")
    }
}

fn to_days(years: u32) -> u32 {
    years * 365
}