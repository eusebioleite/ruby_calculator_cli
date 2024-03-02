use std::env;

fn main() {
    // Retrieve command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if at least one argument (the program name) is provided
    if args.len() != 4 {
        // If no arguments provided (other than the program name), print an error message
        eprintln!("Usage: {} <operation> <x> <y>", args[0]);
        // Exit the program with a non-zero status code to indicate failure
        std::process::exit(1);
    }


    
    let x: f64 = match &args[2].parse::<f64>() {
        Ok(number) => *number,
        Err(_) => {
            eprintln!("Failed to parse x!");
            std::process::exit(1);
        }
    };

    let y: f64 = match &args[3].parse::<f64>() {
        Ok(number) => *number,
        Err(_) => {
            eprintln!("Failed to parse y!");
            std::process::exit(1);
        }
    };

    if x == 0.0 || y == 0.0{
        eprintln!("Division by 0 is not allowed.");
        std::process::exit(1);
    }

    let operation = &args[1];

    match operation.as_str() {
        "a" => println!("Result: {}", (x + y)),
        "s" => println!("Result: {}", (x - y)),
        "d" => if y != 0.0 || x != 0.0 {
                println!("Result: {}", (x / y));
            } else {
                eprintln!("Division by 0 is not allowed.");
                std::process::exit(1);
            },   
        "m" => println!("Result: {}", (x * y)),
        _ => println!("Invalid operation."),
    }

}
