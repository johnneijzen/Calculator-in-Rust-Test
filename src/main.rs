use std::io;

fn main() {
    println!("Hello Welcome to calculator");
    println!("Enter your first number");
    let mut first_number = String::new();
    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read line");

    println!("Enter your Second number");
    let mut second_number = String::new();
    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read line");

    println!("first number {} and second number {}", first_number, second_number);

    println!("Enter Add, Sub, Div, or Mul");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

}
