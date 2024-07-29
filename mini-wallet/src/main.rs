use std::io::{self, stdout, Write};

use mini_wallet::the_system::TheSystem;

fn main() {
    let mut system = TheSystem::default();

    loop {
        print!("\nPlease enter a command: ");
        stdout().flush().unwrap_or_else(|e| {
            println!("unable to flush stdout, {}", e);
        });

        let mut command = String::new();
        if let Err(e) = io::stdin().read_line(&mut command) {
            println!("Failed to read line: {}", e);
            continue;
        }

        let command = command.trim();
        if command.is_empty() {
            println!("given command is empty!");
            continue;
        }

        let parts: Vec<&str> = command.split_whitespace().collect();
        match parts.as_slice() {
            // adding other commands
            ["REGISTER", user_name] => match system.register_user(user_name) {
                Ok(_) => println!("user registered: {}", user_name),
                Err(e) => println!("user unable to register: {}", e),
            },
            ["FETCH_BAL", user_name] => match system.fetch_balance(user_name) {
                Ok(x) => println!("{}'s balance: ", x),
                Err(e) => println!("Unable to fetch: {}", e),
            },
            ["TOP_UP", user_name, src, amount] => match amount.parse::<i32>() {
                Ok(amount) => match system.top_up(user_name, src, amount) {
                    Ok(_) => println!("Amount topped up: {}", amount),
                    Err(e) => println!("unable to top up: {}", e),
                },
                Err(_) => println!("Invalid amount"),
            },
            ["CREATE_DEPOSITE", user_name, account, amount] => match amount.parse::<i32>() {
                Ok(amount) => match system.create_deposite(user_name, account, amount) {
                    Ok(_) => println!("Deposited {} to {}", amount, account),
                    Err(e) => println!("Unable to create deposit: {}", e),
                },
                Err(_) => println!("Invalid amount"),
            },
            ["BOOK_DEPOSITE", user_name, account] => {
                match system.book_deposite(user_name, account) {
                    Ok(_) => println!("All amount from {} added to wallet", account),
                    Err(e) => println!("Unable to book deposit: {}", e),
                }
            }
            ["SEND_MONEY", src, dest, amount] => match amount.parse::<i32>() {
                Ok(amount) => match system.send_money(src, dest, amount) {
                    Ok(_) => println!("Sent {} from {} to {}", amount, src, dest),
                    Err(e) => println!("Unable to send money: {}", e),
                },
                Err(_) => println!("Invalid amount"),
            },
            ["GET_TRANSECTIONS", user_name, cd, sort_by] => {
                match system.get_transections(user_name, cd, sort_by) {
                    Ok(transections) => {
                        for transection in transections {
                            println!("{:?}", transection);
                        }
                    }
                    Err(e) => println!("Unable to get transactions: {}", e),
                }
            }
            ["EXIT"] => {
                println!("Exiting the program.");
                break;
            }
            _ => println!("Invalid command. Please try again."),
        }
    }
}
