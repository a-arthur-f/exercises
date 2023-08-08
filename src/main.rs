mod average;
mod calculator;
mod factorial;
mod interest;
mod palindrome;
mod prime;
mod table;
mod vowel;
use std::io::{self, stdin, Write};

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
                "3" => factorial(),
                "4" => palindrome(),
                "5" => table(),
                "6" => vowel(),
                "7" => average(),
                "8" => interest(),
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
        let mut input = String::new();

        if stdin().read_line(&mut input).is_err() {
            println!("\nThere was an error! Please try again.");
            continue;
        }

        let input = input.as_str().trim();

        if input == "exit" {
            break;
        }

        match calculator::calc(input) {
            Ok(result) => println!("{result}\n"),
            Err(error) => println!("\n{error}\n"),
        }
    }
}

fn prime() {
    loop {
        println!("\nPrime numbers! Choose an option:\n");
        println!("1: Checks if a number is prime.");
        println!("2: Print the first 10 prime numbers.\n");

        let mut option = String::new();

        if stdin().read_line(&mut option).is_err() {
            println!("\nThere was an error! Please try again.");
            continue;
        }

        match option.as_str().trim() {
            "exit" => break,
            "1" => is_prime(),
            "2" => first_10_primes(),
            _ => println!("Invalid option! Please try again."),
        }
    }

    fn is_prime() {
        loop {
            print!("number: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();

            if stdin().read_line(&mut input).is_err() {
                println!("\nThere was an error! Please try again.");
                continue;
            }

            let input = input.as_str().trim();

            if input == "exit" {
                break;
            }

            if let Ok(number) = input.parse::<u64>() {
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
    }

    fn first_10_primes() {
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
}

fn factorial() {
    println!("\nPlease provide a number to calculate its factorial: \n");

    loop {
        print!("number: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        if stdin().read_line(&mut input).is_err() {
            println!("\nThere was an error! Please try again.");
            continue;
        }

        let input = input.as_str().trim();

        if input == "exit" {
            break;
        }

        if let Ok(number) = input.parse::<u32>() {
            match factorial::factorial(number) {
                Ok(factorial) => println!("The factorial of {number} is: {factorial}\n"),
                Err(error) => println!("\n{error}\n"),
            }
        } else {
            println!("\nInvalid number. Please try again.");
        }
    }
}

fn palindrome() {
    println!("\nChecks if an input is palindrome:\n");

    loop {
        print!("input: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        if stdin().read_line(&mut input).is_err() {
            println!("\nThere was an error! Please try again.");
            continue;
        }

        let input = input.trim();

        if input == "exit" {
            break;
        }

        let result = {
            if palindrome::is_palindrome(input) {
                "is palindrome"
            } else {
                "is not palindrome"
            }
        };

        println!("{input} {result}\n");
    }
}

fn table() {
    println!("\nTable of a number! Choose an option.\n");

    loop {
        println!("1: Table from 1 to 10");
        println!("2: Custom table\n");

        let mut option = String::new();

        match stdin().read_line(&mut option) {
            Ok(_) => {
                let option = option.trim();
                match option {
                    "1" => loop {
                        print!("number: ");
                        io::stdout().flush().unwrap();
                        let mut input = String::new();

                        match stdin().read_line(&mut input) {
                            Ok(_) => {
                                let input = input.trim();
                                if input == "exit" {
                                    break;
                                }
                                if let Ok(number) = input.parse::<f32>() {
                                    let table = table::Table::gen(number, 1..=10);
                                    println!("\n{table}");
                                } else {
                                    println!("Invalid number! Please try again.")
                                }
                            }

                            Err(_) => println!("There was an error! Please try again."),
                        }
                    },

                    "2" => loop {
                        print!("number: ");
                        io::stdout().flush().unwrap();

                        let mut input = String::new();

                        if stdin().read_line(&mut input).is_err() {
                            println!("There was an error! Please try again.");
                            continue;
                        }

                        let input = input.trim();

                        if input == "exit" {
                            break;
                        }

                        if let Ok(number) = input.parse::<f32>() {
                            print!("from: ");
                            io::stdout().flush().unwrap();
                            let mut input = String::new();
                            if stdin().read_line(&mut input).is_err() {
                                println!("There was an error! Please try again.");
                                continue;
                            }

                            let input = input.trim();

                            if let Ok(from) = input.parse::<usize>() {
                                print!("to: ");
                                io::stdout().flush().unwrap();
                                let mut input = String::new();
                                if stdin().read_line(&mut input).is_err() {
                                    println!("There was an error! Please try again.");
                                    continue;
                                }

                                let input = input.trim();

                                if let Ok(to) = input.parse::<usize>() {
                                    let table = table::Table::gen(number, from..=to);
                                    println!("\n{table}\n");
                                } else {
                                    println!("Invalid number! Please try again.")
                                }
                            } else {
                                println!("Invalid number! Please try again.")
                            }
                        } else {
                            println!("Invalid number! Please try again.");
                        }
                    },

                    "exit" => {
                        break;
                    }

                    _ => println!("Invalid option! Please try again."),
                }
            }

            Err(_) => println!("There was an error. Please try again."),
        }
    }
}

fn vowel() {
    println!("Count vowels in a string!\n");
    loop {
        print!("input: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        if stdin().read_line(&mut input).is_err() {
            println!("There was an error! Please try again");
            continue;
        }

        let input = input.trim();

        if input == "exit" {
            break;
        }

        let (count, vowels) = vowel::count(&input);
        println!("\ntotal: {count}");
        println!("a: {}", vowels.get(&'a').unwrap());
        println!("e: {}", vowels.get(&'e').unwrap());
        println!("i: {}", vowels.get(&'i').unwrap());
        println!("o: {}", vowels.get(&'o').unwrap());
        println!("u: {}\n", vowels.get(&'u').unwrap());
    }
}

fn average() {
    println!("Grade average!\n");
    loop {
        print!("grade 1: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        if stdin().read_line(&mut input).is_err() {
            println!("There was an error! Please try again.");
            continue;
        }

        let input = input.trim();

        if input == "exit" {
            break;
        }

        if let Ok(grade_1) = input.parse::<f32>() {
            print!("grade 2: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();

            if stdin().read_line(&mut input).is_err() {
                println!("There was an error! Please try again.");
                continue;
            }

            let input = input.trim();

            if let Ok(grade_2) = input.parse::<f32>() {
                print!("grade 3: ");
                io::stdout().flush().unwrap();

                let mut input = String::new();

                if stdin().read_line(&mut input).is_err() {
                    println!("There was an error! Please try again.");
                    continue;
                }

                let input = input.trim();

                if let Ok(grade_3) = input.parse::<f32>() {
                    let mut student = average::Student::new();
                    student.new_grade(average::Grade(grade_1));
                    student.new_grade(average::Grade(grade_2));
                    student.new_grade(average::Grade(grade_3));

                    let average = student.get_average();

                    println!("The average is: {average}\n");
                    continue;
                }
            }
        }

        println!("Invalid number! Please try again.\n");
    }
}

fn interest() {
    println!("Interest calculator!\n");
    loop {
        print!("initial capital: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        if stdin().read_line(&mut input).is_err() {
            println!("There was an error! Please try again.");
            continue;
        }

        let input = input.trim();

        if input == "exit" {
            break;
        }

        if let Ok(initial_capital) = input.parse::<f32>() {
            print!("interest rate(decimal): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();

            if stdin().read_line(&mut input).is_err() {
                println!("There was an error! Please try again.");
                continue;
            }

            let input = input.trim();

            if let Ok(interest_rate) = input.parse::<f32>() {
                print!("investment time(months): ");
                io::stdout().flush().unwrap();

                let mut input = String::new();

                if stdin().read_line(&mut input).is_err() {
                    println!("There was an error! Please try again.");
                    continue;
                }

                let input = input.trim();

                if let Ok(investment_time) = input.parse::<u32>() {
                    let interest =
                        interest::Interest::new(initial_capital, interest_rate, investment_time);

                    let total_interest = interest.total_interest();
                    let final_value = interest.final_value();

                    println!("\ntotal interest: {total_interest}");
                    println!("final value: {final_value}\n");
                    continue;
                }
            }
        }

        println!("Invalid number! Please try again.\n");
    }
}
