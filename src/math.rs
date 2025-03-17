use std::collections::HashMap;
use crate::Value;
use num_complex::Complex;

pub enum MathValue {
    Int(i32),
    Float(f64),
    Complex(Complex<f64>),
}

// Parse and execute math operations, handling variables
pub fn evaluate_math(expr: &str, variables: &HashMap<String, Value>) -> Option<i32> {
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    
    match tokens.as_slice() {
        // Basic arithmetic (existing functionality)
        [left, op, right] => {
            let left = get_number_value(left, variables)?;
            let right = get_number_value(right, variables)?;
            
            match *op {
                "+" => Some(left + right),
                "-" => Some(left - right),
                "*" => Some(left * right),
                "/" => Some(left / right),
                "^" => Some(left.pow(right as u32)),
                "%" => Some(left % right),
                _ => None,
            }
        },
        
        // Advanced math functions
        ["pow", base, exp] => {
            let base = get_number_value(base, variables)?;
            let exp = get_number_value(exp, variables)?;
            Some(base.pow(exp as u32))
        },
        ["sqrt", num] => {
            let num = get_number_value(num, variables)?;
            Some((num as f64).sqrt() as i32)
        },
        ["log", num, base] => {
            let num = get_number_value(num, variables)?;
            let base = get_number_value(base, variables)?;
            Some((num as f64).log(base as f64) as i32)
        },
        ["ln", num] => {
            let num = get_number_value(num, variables)?;
            Some((num as f64).ln() as i32)
        },
        ["abs", num] => {
            let num = get_number_value(num, variables)?;
            Some(num.abs())
        },
        _ => None,
    }
}

fn get_number_value(token: &str, variables: &HashMap<String, Value>) -> Option<i32> {
    if let Ok(num) = token.parse::<i32>() {
        Some(num)
    } else if let Some(Value::Int(num)) = variables.get(token) {
        Some(*num)
    } else {
        None
    }
}