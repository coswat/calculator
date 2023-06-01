#![allow(unused)]

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
use std::io;

/// Main function that prints a welcome message and runs the calculator.
fn main() {
    println!("Welcome to simple DMAS Calculater");
    run_calculater();
}
/// Runs the calculator by taking user input for numbers and operator.
fn run_calculater() {
    println!("{}Enter First Number :{}", "\x1b[32m", "\x1b[0m");
    let mut first: String = String::new();
    let mut second: String = String::new();
    let mut opr: String = String::new();
    io::stdin().read_line(&mut first);
    // Parse the first number
    let first: i32 = match first.trim().parse() {
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
    io::stdin().read_line(&mut second);
    // Parse the second number
    let second: i32 = match second.trim().parse() {
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
    io::stdin().read_line(&mut opr);
    // Check the validity of numbers and perform the calculation
    if first > i32::MAX {
        println!(
            "{}First number must be between 1 - {} , {} given{}",
            "\x1b[31m",
            i32::MAX,
            first,
            "\x1b[0m"
        );
    } else if second > i32::MAX {
        println!(
            "{}Second number must be between 1 - {} , {} given{}",
            "\x1b[31m",
            i32::MAX,
            second,
            "\x1b[0m"
        );
    } else {
        calculate_number(&opr, first, second);
    }
}
/// Calculates the result based on the operator and prints it.
/// Supports addition (+), subtraction (-), multiplication (*), and division (/).
fn calculate_number(opr: &str, first: i32, second: i32) {
    match opr.trim() {
        // Addition operatiom
        "+" => println!("Addition {} + {} = {}", first, second, first + second),
        // Substraction operatiom
        "-" => println!("Substraction {} - {} = {}", first, second, first - second),
        // Multiplication operatiom
        "*" => println!("Multiplication {} * {} = {}", first, second, first * second),
        // Division operatiom
        "/" => println!("Division {} / {} = {}", first, second, first / second),
        // Handle th error for invalid operater
        _ => {
            println!(
                "{}Invalid operater {} , supported [/, *, +, -]{}",
                "\x1b[31m", opr, "\x1b[0m"
            );
            return;
        }
    };
}
