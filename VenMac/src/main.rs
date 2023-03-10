use std::io;

fn main() {

    let mut balance = 0;

    println!("Welcome! Your current balance is: {balance}"); 

    println!("Please input the number you want to add");

    let mut amount = String::new();

    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read input");
    
    let amount: u64 = amount.trim().parse().expect("Please type a number"); 

    fn add_money(amount: u64, balance: u64) -> u64 {
        balance + amount
    }

    balance = add_money(amount, balance);

    println!("Available balance is: {balance}");
}
