#![allow(unused)]
#![allow(non_snake_case)]

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
    runCalculater();
}
/// Runs the calculator by taking user input for numbers and operator.
fn runCalculater() {
    println!("Enter First Number :");
    let mut first: String = String::new();
    let mut second: String = String::new();
    let mut opr: String = String::new();
    io::stdin().read_line(&mut first);
    // Parse the first number
    let first: i32 = match first.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("First number should be a valid number");
            return;
        }
    };
    println!("Enter Second Number :");
    io::stdin().read_line(&mut second);
    // Parse the second number
    let second: i32 = match second.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Second number should be a valid number");
            return;
        }
    };
    println!("Calculation operater : [/, *, +, -]");
    io::stdin().read_line(&mut opr);
    // Check the validity of numbers and perform the calculation
    if first > i32::MAX {
        println!(
            "First number must be between 1 - {} , {} given",
            i32::MAX,
            first
        );
    } else if second > i32::MAX {
        println!(
            "Second number must be between 1 - {} , {} given",
            i32::MAX,
            second
        );
    } else {
        calculateNumber(&opr, first, second);
    }
}
/// Calculates the result based on the operator and prints it.
/// Supports addition (+), subtraction (-), multiplication (*), and division (/).
fn calculateNumber(opr: &str, first: i32, second: i32) {
    match opr.trim() {
        "+" => println!("Addition {} + {} = {}", first, second, first + second),
        "-" => println!("Substraction {} - {} = {}", first, second, first - second),
        "*" => println!("Multiplication {} * {} = {}", first, second, first * second),
        "/" => println!("Division {} / {} = {}", first, second, first / second),
        _ => {
            println!("Invalid operater {} , supported [/, *, +, -]", opr);
            return;
        }
    };
}
