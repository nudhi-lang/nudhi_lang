use crate::HashMap;
use crate::Value;

// Parse and execute math operations, handling variables
pub fn evaluate_math(expr: &str, variables: &HashMap<String, Value>) -> Option<i32> {
    // Replace variable names with their integer values in the expression
    let mut expression = expr.to_string();
    for (key, value) in variables {
        if let Value::Int(val) = value {
            expression = expression.replace(key, &val.to_string());
        }
    }

    // Split by basic operators and handle math
    let tokens: Vec<&str> = expression.split_whitespace().collect();
    if tokens.len() == 3 {
        let left = tokens[0].parse::<i32>().ok()?;
        let right = tokens[2].parse::<i32>().ok()?;
        let result = match tokens[1] {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            _ => return None,
        };
        Some(result)
    } else {
        None
    }
}