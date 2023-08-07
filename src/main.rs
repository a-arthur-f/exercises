mod average;
mod calculator;
mod factorial;
mod interest;
mod prime;
mod table;
mod vowel;
use std::io::stdin;

fn main() {
    println!("\nWelcome! Choose an option:");
    loop {
        println!("\n1: calculator");
        println!("2: prime numbers");
        println!("3: factorial");
        println!("4: palindrome");
        println!("5: table");
        println!("6: vowel counter");
        println!("7: grade average");
        println!("8: interest calculation");
        println!("\ntype \"exit\" to exit\n");

        let mut option = String::new();

        match stdin().read_line(&mut option) {
            Ok(_) => match option.as_str().trim() {
                "1" => calculator(),
                "2" => prime(),
                "3" => {}
                "4" => {}
                "5" => {}
                "6" => {}
                "7" => {}
                "8" => {}
                "exit" => break,
                _ => {
                    println!("\nInvalid option! Pleasy try again.\n");
                }
            },

            Err(_) => println!("\nThere was an error! Please try again.\n"),
        }
    }
}

fn calculator() {
    println!("\n# Write a mathematical expression (operators separated by space)");
    println!("# Does not support parentheses\n");

    loop {
        let mut expr = String::new();

        match stdin().read_line(&mut expr) {
            Ok(_) => match expr.as_str().trim() {
                "exit" => break,
                _ => match calculator::calc(&expr) {
                    Ok(result) => println!("{result}\n"),
                    Err(error) => println!("\n{error}\n"),
                },
            },

            Err(_) => {
                println!("\nThere was an error! Please try again.");
            }
        }
    }
}

fn prime() {
    loop {
        println!("\nPrime numbers! Choose an option:\n");
        println!("1: Checks if a number is prime.");
        println!("2: Print the first 10 prime numbers.\n");

        let mut option = String::new();

        match stdin().read_line(&mut option) {
            Ok(_) => match option.as_str().trim() {
                "1" => loop {
                    let mut number = String::new();

                    match stdin().read_line(&mut number) {
                        Ok(_) => {
                            if let Ok(number) = number.trim().parse::<u64>() {
                                let result = {
                                    if prime::is_prime(number, 40) {
                                        format!("{number} is prime")
                                    } else {
                                        format!("{number} is not prime")
                                    }
                                };

                                println!("{result}");
                            } else {
                                println!("Invalid number! Please try again.")
                            }
                        }

                        Err(_) => println!("\nThere was an error! Please try again."),
                    }
                },
                "2" => {
                    println!("\nFirst 10 prime numbers:");

                    let mut found_primes = 0; 
                    let mut current_number = 2u64;

                    while found_primes < 10 {
                        if prime::is_prime(current_number, 40) {
                            found_primes += 1;
                            println!("{current_number}");
                        }
                        current_number += 1;
                    }
                }
                _ => println!("Invalid option! Please try again."),
            },

            Err(_) => println!("\nThere was an error! Please try again."),
        }
    }
}
