use std::io;

fn main() {

    let mut balance = 0;

    println!("Current balance is: {balance}"); 

    println!("Please input the number you want to add");

    

    fn add_money(amount: u64, balance: u64) -> u64 {
        balance + amount
    }

    balance = add_money(300, balance);

    println!("Available balance is: {balance}");
}
