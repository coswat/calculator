use std::env::args;
use std::io;

/**
 * Simple DMAS Calculator
 *
 * This program allows the user to perform basic arithmetic calculations using two numbers.
 * The supported operations are addition (+), subtraction (-), multiplication (*), and division (/).
 * The user is prompted to enter the first and second numbers, as well as the operator.
 * The program then performs the calculation and displays the result.
 *
 * Example usage:
 * ```
 * Welcome to simple DMAS Calculator
 * Enter First Number:
 * 10
 * Enter Second Number:
 * 5
 * Calculation operator: [/, *, +, -]
 * *
 * Multiplication 10 * 5 = 50
 * ```
 *
 * Author: coswat
 * Date: 2023-06-02
 */

/// Main function that prints a welcome message and runs the calculator.
fn main() {
    // collect the args
    let args: Vec<String> = args().collect();
    // if the caclculation through args , proccess goes here
    if args.len() > 3 {
        let ans: f32 = calculate_number(
            &args[2],
            args[1].trim().parse().unwrap(),
            args[3].trim().parse().unwrap(),
        );
        println!("{} {} {} = {}", args[1], args[2], args[3], ans);
        return;
    }
    println!("Welcome to simple DMAS Calculater");
    // else run normal calculator
    run_calculater();
}
/// Runs the calculator by taking user input for numbers and operator.
fn run_calculater() {
    println!("{}Enter First Number :{}", "\x1b[32m", "\x1b[0m");
    let mut first: String = String::new();
    let mut second: String = String::new();
    let mut opr: String = String::new();
    io::stdin()
        .read_line(&mut first)
        .expect("unable to read line");
    // Parse the first number
    let first: f32 = match first.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!(
                "{}First number should be a valid number{}",
                "\x1b[31m", "\x1b[0m"
            );
            return;
        }
    };
    println!("{}Enter Second Number :{}", "\x1b[32m", "\x1b[0m");
    io::stdin()
        .read_line(&mut second)
        .expect("unable to read line");
    // Parse the second number
    let second: f32 = match second.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!(
                "{}Second number should be a valid number{}",
                "\x1b[31m", "\x1b[0m"
            );
            return;
        }
    };
    println!(
        "{}Calculation operater : [/, *, +, -]{}",
        "\x1b[32m", "\x1b[0m"
    );
    io::stdin()
        .read_line(&mut opr)
        .expect("unable to read line");
    // Check the validity of numbers and perform the calculation
    if first > f32::MAX {
        println!(
            "{}First number must be between 1 - {} , {} given{}",
            "\x1b[31m",
            f32::MAX,
            first,
            "\x1b[0m"
        );
    } else if second > f32::MAX {
        println!(
            "{}Second number must be between 1 - {} , {} given{}",
            "\x1b[31m",
            f32::MAX,
            second,
            "\x1b[0m"
        );
    } else {
        let ans: f32 = calculate_number(&opr, first, second);
        println!("{} {} {} = {}", first, opr, second, ans);
    }
}
/// Calculates the result based on the operator and prints it.
/// Supports addition (+), subtraction (-), multiplication (*), and division (/).
fn calculate_number(opr: &str, first: f32, second: f32) -> f32 {
    match opr.trim() {
        // Addition operatiom
        "+" => first + second,
        // Substraction operatiom
        "-" => first - second,
        // Multiplication operatiom
        "*" => first * second,
        // Division operatiom
        "/" => {
            // check the second number is 0
            if second == 0.0 {
                return 0.0;
            }
            first / second
        }
        // Handle th error for invalid operater*/
        _ => return 0.0,
    }
}
