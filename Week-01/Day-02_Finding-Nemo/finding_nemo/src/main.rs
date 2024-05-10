use std::io;

fn main() {
    println!("Please input your string");

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    // enumerate() retorna una tupla  (i, val)
    for (i, word) in line.split_whitespace().enumerate() {
        if word.eq("Nemo") {
            println!("I found Nemo at {}!", i+1);
            return;
        }
    }

    println!("I can't find \"Nemo\" :(");
}