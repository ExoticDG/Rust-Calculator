
use std::{env, io};

fn os() {
    let mut ostype = String::new();
    println!(
        "Hello! It looks like you are using the {} os. Is this correct? (Y/N)",
        env::consts::OS
    );

    ostype.clear();
    io::stdin().read_line(&mut ostype).unwrap();
    ostype = ostype.trim().to_string();

    if ostype.to_lowercase() == "y" {
        if cfg!(windows) {
            if env::consts::OS == "windows" {
                println!("Thanks for confirming.");
            }
            } else if cfg!(unix) {
                if env::consts::OS == "linux" {
                    println!("Thanks for confirming.");
                } else {
                    panic!("Exit Code 837 There is a problem with OS detection.");
                }
            }
        
    } else if ostype.to_lowercase() == "n" {
        if env::consts::OS == "windows" {
            println!("This is Windows mate. Stop trying to be funny. Your not.");
        } else if cfg!(unix) {
            if env::consts::OS == "linux" {
                println!("This is Linux mate. Stop trying to be funny. Your not.");
            } else {
                panic!("Exit Code 837. There is a problem with OS detection.");
            }
        }
    }
}

fn main() {
    os();

    let mut sym = String::new();
    let mut num1_string = String::new();
    let mut num2_string = String::new();

    println!(
        "Please type the first number and hit enter. (if you want to use pi type 3.14 in this)"
    );

    num1_string.clear();
    io::stdin().read_line(&mut num1_string).unwrap();
    num1_string = num1_string.trim().to_string();
    let num1 = num1_string.parse::<f64>().unwrap();

    println!("You selected {}", num1_string);

    println!(
        "Please type the second number and hit enter. (if you want to use pi type 3.14 in this)"
    );

    num2_string.clear();
    io::stdin().read_line(&mut num2_string).unwrap();
    num2_string = num2_string.trim().to_string();
    let num2: f64 = num2_string.parse().unwrap();

    println!("You selected {}", num2_string);

    println!("Please type a math symbol (* , / , + , -) and hit enter.");

    sym.clear();
    io::stdin().read_line(&mut sym).unwrap();
    sym = sym.trim().to_string();

    println!("You selected {}", sym);

    solve(num1, sym, num2);
}

fn solve(num1: f64, sym: String, num2: f64) {
    let mut answer: f64;
    let pi1: f64;
    let pi2: f64;

    if num2 == 3.14 {
        pi2 = std::f64::consts::PI;

        if sym == "*" {
            answer = num1 * pi2;
            println!("The answer is {}", answer);
        } else if sym == "/" {
            answer = num1 / pi2;
            println!("The answer is {}", answer);
        } else if sym == "-" {
            answer = num1 - pi2;
            println!("The answer is {}", answer);
        } else if sym == "+" {
            answer = num1 + pi2;
            println!("The answer is {}", answer);
        } else {
            println!("Sorry. This is not a viable problem. Try again.")
        };
    }

    if num1 == 3.14 {
        pi1 = std::f64::consts::PI;
        println!("{}", pi1);

        if sym == "*" {
            answer = pi1 * num2;
            println!("The answer is {}", answer);
        } else if sym == "/" {
            answer = pi1 / num2;
            println!("The answer is {}", answer);
        } else if sym == "-" {
            answer = pi1 - num2;
            println!("The answer is {}", answer);
        } else if sym == "+" {
            answer = pi1 + num2;
            println!("The answer is {}", answer);
        } else {
            println!("Sorry. This is not a viable problem. Try again.")
        };
    }

    if num1 == 3.14 {
    } else if num2 == 3.14 {
    } else {
        if sym == "*" {
            answer = num1 * num2;
            println!("The answer is {}", answer);
        } else if sym == "/" {
            answer = num1 / num2;
            println!("The answer is {}", answer);
        } else if sym == "-" {
            answer = num1 - num2;
            println!("The answer is {}", answer);
        } else if sym == "+" {
            answer = num1 + num2;
            println!("The answer is {}", answer);
        } else {
            println!("Sorry. This is not a viable problem. Try again.")
        };
    }

    rerun()
}

fn rerun() {
    let mut rerun = String::new();
    println!("Would you like to solve another problem? (y/n)");
    rerun.clear();
    io::stdin().read_line(&mut rerun).unwrap();
    rerun = rerun.trim().to_string();

    if rerun.to_lowercase() == "y" {
        println!("ok.");
        main();
    } else if rerun.to_lowercase() == "n" {
        println!("Bye.");
        std::process::exit(0)
    } else {
        panic!("Exit Code 317. That is not a viable input.");
    }
}

