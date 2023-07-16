use std::io::{self, Write};

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

fn main() {

    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = String::new();


    print!("What is the first number: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut num1).unwrap();
    let first_number: f64 = num1.trim().parse().unwrap();

    print!("What is the second number: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut num2).unwrap();
    let second_number: f64 = num2.trim().parse().unwrap();

    print!("What is the operation (+, -, *, /): ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut operator).unwrap();

    let operation = match operator.trim() {
        "+" => Operation::Add,
        "-" => Operation::Subtract,
        "*" => Operation::Multiply,
        "/" => Operation::Divide,
        _ => panic!("Error"),
    };

    let result = calculate(operation(first_number, second_number));

    println!("Result: {}", result);
}
