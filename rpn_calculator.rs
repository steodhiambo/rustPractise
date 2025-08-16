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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_addition() {
        assert_eq!(rpn("3 4 +"), "7");
    }

    #[test]
    fn test_simple_subtraction() {
        assert_eq!(rpn("10 3 -"), "7");
    }

    #[test]
    fn test_simple_multiplication() {
        assert_eq!(rpn("3 4 *"), "12");
    }

    #[test]
    fn test_simple_division() {
        assert_eq!(rpn("12 3 /"), "4");
    }

    #[test]
    fn test_simple_modulo() {
        assert_eq!(rpn("10 3 %"), "1");
    }

    #[test]
    fn test_complex_expression_1() {
        // 1 2 * 3 * 4 +
        // 2 3 * 4 +
        // 6 4 +
        // 10
        assert_eq!(rpn("1 2 * 3 * 4 +"), "10");
    }

    #[test]
    fn test_complex_expression_2() {
        // 1 2 * 3 * 4 -
        // 2 3 * 4 -
        // 6 4 -
        // 2
        assert_eq!(rpn("1 2 * 3 * 4 -"), "2");
    }

    #[test]
    fn test_with_extra_spaces() {
        assert_eq!(rpn("     1      3 * 2 -"), "1");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(rpn(""), "Error");
    }

    #[test]
    fn test_insufficient_operands() {
        assert_eq!(rpn("1 2 3 4 +"), "Error");
    }

    #[test]
    fn test_invalid_token() {
        assert_eq!(rpn("1 3 * ksd 2 -"), "Error");
    }

    #[test]
    fn test_division_by_zero() {
        assert_eq!(rpn("5 0 /"), "Error");
    }

    #[test]
    fn test_modulo_by_zero() {
        assert_eq!(rpn("5 0 %"), "Error");
    }

    #[test]
    fn test_single_number() {
        assert_eq!(rpn("42"), "42");
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(rpn("-5 3 +"), "-2");
        assert_eq!(rpn("5 -3 +"), "2");
    }

    #[test]
    fn test_too_many_operators() {
        assert_eq!(rpn("1 +"), "Error");
    }

    #[test]
    fn test_too_many_operands() {
        assert_eq!(rpn("1 2 3"), "Error");
    }

    #[test]
    fn test_complex_valid_expression() {
        // 5 10 9 / - 50 *
        // 5 1 - 50 * (since 10/9 = 1 in integer division)
        // 4 50 *
        // 200
        assert_eq!(rpn("5 10 9 / - 50 *"), "200");
    }
}
