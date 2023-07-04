use colored::*;
use std::env;
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
    let args: Vec<String> = env::args().collect();
    // if the caclculation through args , proccess goes here
    if env::args().len() > 3 {
        match calculate_number(
            &args[2],
            args[1].trim().parse().unwrap(),
            args[3].trim().parse().unwrap(),
        ) {
            Ok(ans) => {
                println!("{}", ans);
                return;
            }
            Err(err) => {
                println!("{}", err.red());
                return;
            }
        }
    }
    println!("Welcome to simple DMAS Calculater");
    // else run normal calculator
    run_calculater();
}
/// Runs the calculator by taking user input for numbers and operator.
fn run_calculater() {
    println!("{}", "Enter First Number :".green());
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
            println!("{}", "First number should be a valid number".red());
            return;
        }
    };
    println!("{}", "Enter Second Number :".green());
    io::stdin()
        .read_line(&mut second)
        .expect("unable to read line");
    // Parse the second number
    let second: f32 = match second.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "Second number should be a valid number".red());
            return;
        }
    };
    println!("{}", "Calculation operater : [/, *, +, -]".green());
    io::stdin()
        .read_line(&mut opr)
        .expect("unable to read line");

    match calculate_number(&opr, first, second) {
        Ok(ans) => println!("{}", ans),
        Err(err) => println!("{}", err.red()),
    }
}
/// Calculates the result based on the operator and prints it.
/// Supports addition (+), subtraction (-), multiplication (*), and division (/).
fn calculate_number(opr: &str, first: f32, second: f32) -> Result<f32, String> {
    match opr.trim() {
        // Addition operatiom
        "+" => Ok(first + second),
        // Substraction operatiom
        "-" => Ok(first - second),
        // Multiplication operatiom
        "*" => Ok(first * second),
        // Division operatiom
        "/" => {
            // check the second number is 0
            if second == 0.0 {
                return Err(String::from("Number cannot divisible by 0"));
            }
            Ok(first / second)
        }
        // Handle th error for invalid operater*/
        _ => Err(String::from("Invalid Operater")),
    }
}
