use std::io;

fn main() {
    println!("Choose notation type (normal/polish):");
    let mut choice: String = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: String = choice.trim().to_lowercase();

    let result_pack: Result<(f64, f64, String), String> = match choice.as_str() {
        "normal" => process_normal_notation(),
        "polish" => process_polish_notation(),
        _ => {
            println!("Invalid choice. Please choose \"normal\" or \"polish\".");
            return;
        }
    };
    let result: (f64, f64, String);
    match result_pack {
        Ok(value) => result = value,
        Err(value) => {
            println!("{}", value);
            return;
        }
    }

    let result = match result.2.as_str() {
        "+" => result.0 + result.1,
        "-" => result.0 - result.1,
        "*" => result.0 * result.1,
        "/" => {
            if result.1 == 0.0 {
                println!("Error: Division by zero is not allowed.");
                return;
            }
            result.0 / result.1
        }
        _ => {
            println!("Invalid operation. Please use one of +, -, *, /.");
            return;
        }
    };
    println!("{}", result);
}

fn process_normal_notation() -> Result<(f64, f64, String), String> {
    println!("Enter an expression:");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 3 {
        return Err(String::from("Invalid input format. Please use the format: \
        <number> <operator> <number>"));
    }

    let num1: f64 = match parts[0].parse() {
        Ok(value) => value,
        Err(_e) => {
            return Err(String::from("Error at Number 1 validation"))
        }
    };

    let operation: &str = parts[1];
    let num2: f64 = match parts[2].parse() {
        Ok(value) => value,
        Err(_e) => {
            return Err(String::from("Error at Number 2 validation"))
        }
    };
    Ok((num1, num2, String::from(operation)))

}

fn process_polish_notation() -> Result<(f64, f64, String), String> {
    println!("Enter an expression:");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 3 {
        return Err(String::from("Invalid input format. Please use the format: \
        <operator> <number> <number>"));
    }

    let operation: &str = parts[0];
    let num1: f64 = match parts[1].parse() {
        Ok(value) => value,
        Err(_e) => {
            return Err(String::from("Error at Number 1 validation"))
        }
    };
    let num2: f64 = match parts[2].parse() {
        Ok(value) => value,
        Err(_e) => {
            return Err(String::from("Error at Number 2 validation"))
        }
    };
    Ok((num1, num2, String::from(operation)))
}
