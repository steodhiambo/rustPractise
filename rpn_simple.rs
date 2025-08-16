pub fn rpn(expression: &str) -> String {
    let tokens: Vec<&str> = expression.split_whitespace().collect();
    
    // Empty expression is invalid
    if tokens.is_empty() {
        return "Error".to_string();
    }
    
    let mut stack: Vec<i64> = Vec::new();
    
    for token in tokens {
        match token {
            "+" | "-" | "*" | "/" | "%" => {
                // Need at least 2 operands for any operation
                if stack.len() < 2 {
                    return "Error".to_string();
                }
                
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                
                let result = match token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => {
                        if b == 0 {
                            return "Error".to_string(); // Division by zero
                        }
                        a / b
                    },
                    "%" => {
                        if b == 0 {
                            return "Error".to_string(); // Modulo by zero
                        }
                        a % b
                    },
                    _ => unreachable!(),
                };
                
                stack.push(result);
            },
            _ => {
                // Try to parse as number
                match token.parse::<i64>() {
                    Ok(num) => stack.push(num),
                    Err(_) => return "Error".to_string(), // Invalid token
                }
            }
        }
    }
    
    // Must have exactly one result left on the stack
    if stack.len() == 1 {
        stack[0].to_string()
    } else {
        "Error".to_string()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 2 {
        println!("Error");
        return;
    }
    
    let result = rpn(&args[1]);
    println!("{}", result);
}
