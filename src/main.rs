use colored::*;
use std::env;
use std::io;
use std::process;

/**
/// Simple DMAS Calculator
///
/// This program allows the user to perform basic arithmetic calculations using two numbers.
/// The supported operations are addition (+), subtraction (-), multiplication (*), and division (/).
/// The user is prompted to enter the first and second numbers, as well as the operator.
/// The program then performs the calculation and displays the result.
///
/// Example usage:
/// ```
/// Welcome to simple DMAS Calculator
/// Enter First Number:
/// 10
/// Enter Second Number:
/// 5
/// Calculation operator: [/, *, +, -]
///
/// Multiplication 10 * 5 = 50
/// ```
///
/// Author: coswat
/// Date: 2023-06-02
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
                process::exit(0);
            }
            Err(err) => {
                println!("{}", err.red());
                process::exit(1);
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
    let first: f32 = first.trim().parse().unwrap_or_else(|_err| {
        eprintln!("{}", "First number should be a valid number".red());
        process::exit(1);
    });

    println!("{}", "Enter Second Number :".green());
    io::stdin()
        .read_line(&mut second)
        .expect("unable to read line");
    // Parse the second number
    let second: f32 = second.trim().parse().unwrap_or_else(|_err| {
        eprintln!("{}", "Second number should be a valid number".red());
        process::exit(1);
    });
    println!("{}", "Calculation operater : [/, *, +, -]".green());
    io::stdin()
        .read_line(&mut opr)
        .expect("unable to read line");

    match calculate_number(&opr, first, second) {
        Ok(ans) => {
            println!("{}", ans);
            process::exit(0);
        }
        Err(err) => {
            eprintln!("{}", err.red());
            process::exit(1);
        }
    }
}
/// Calculates the result based on the operator and prints it.
/// Supports addition (+), subtraction (-), multiplication (*), and division (/).
fn calculate_number(opr: &str, first: f32, second: f32) -> Result<f32, &str> {
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
                return Err("Number cannot divisible by 0");
            }
            Ok(first / second)
        }
        // Handle th error for invalid operater*/
        _ => Err("Invalid Operater"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_number() {
        let opr: &str = "+";
        let first: f32 = 10.0;
        let second: f32 = 15.0;
        assert_eq!(calculate_number(opr, first, second), Ok(25.0));
    }
    #[test]
    fn test_invalid_operater_err() {
        let opr: &str = "invalid";
        let first: f32 = 10.0;
        let second: f32 = 15.0;
        assert_eq!(
            calculate_number(opr, first, second),
            Err("Invalid Operater")
        );
    }
    #[test]
    fn test_number_not_divisible_by_zero() {
        let opr: &str = "/";
        let first: f32 = 170.0;
        let second: f32 = 0.0;
        assert_eq!(
            calculate_number(opr, first, second),
            Err("Number cannot divisible by 0")
        );
    }
}
