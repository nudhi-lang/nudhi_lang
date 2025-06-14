#![allow(warnings)]

mod files;
mod math;
mod strings;

use crate::math::evaluate_math;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::exit;
use std::process::Command;
use std::io;
use winreg::enums::*;
use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

// Structure to store variables (as strings or integers)
enum Value {
    Int(i32),
    Str(String),
}

// nudhi_say: print text or integers
fn nudhi_say(trimmed_line: &str, variables: &HashMap<String, Value>) {
    let parts: Vec<&str> = trimmed_line.split_whitespace().collect();

    // Check if the line contains a quoted string
    if let Some(start) = trimmed_line.find('"') {
        if let Some(end) = trimmed_line.rfind('"') {
            if start != end {
                let message = &trimmed_line[start + 1..end]; // Extract the string between the quotes
                println!("{}", message);
            } else {
                eprintln!("Error: Unterminated string");
            }
        } else {
            eprintln!("Error: Missing closing quote");
        }
    } else if parts.len() == 2 {
        // If no quotes are found, check if it's a variable name
        let var_name: &str = parts[1];
        if let Some(value) = variables.get(var_name) {
            match value {
                Value::Int(val) => println!("{}", val),
                Value::Str(val) => println!("{}", val),
            }
        } else {
            eprintln!("Error: Variable '{}' not found", var_name);
        }
    } else {
        eprintln!("Error: Incorrect syntax for nudhi_say");
    }
}

// nudhi_do: execute system command
fn nudhi_do(trimmed_line: &str) {
    if let Some(start) = trimmed_line.find('"') {
        if let Some(end) = trimmed_line.rfind('"') {
            if start != end {
                let message = &trimmed_line[start + 1..end];
                Command::new("cmd")
                    .args(["/c", &message.replace(" ", "%space%")])
                    .status()
                    .expect("failed to execute process");
            } else {
                eprintln!("Error: Unterminated string");
            }
        } else {
            eprintln!("Error: Missing closing quote");
        }
    } else {
        eprintln!("Error: Missing opening quote");
    }
}

// nudhi_ask: take input and store it under the variable name provided
fn nudhi_ask(trimmed_line: &str, variables: &mut HashMap<String, Value>) {
    // Extract the prompt (everything between the first and last quote)
    if let Some(start) = trimmed_line.find('"') {
        if let Some(end) = trimmed_line.rfind('"') {
            if start != end {
                let prompt = &trimmed_line[start + 1..end]; // Extract the string between quotes
                println!("{}", prompt); // Print the prompt to the user

                // Extract the variable name, which comes after the last quote
                let var_name = trimmed_line[end + 1..].trim();

                // Read user input
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let input = input.trim().to_string();

                // Store the input as either an integer or a string
                if let Ok(int_val) = input.parse::<i32>() {
                    variables.insert(var_name.to_string(), Value::Int(int_val));
                } else {
                    variables.insert(var_name.to_string(), Value::Str(input));
                }
            } else {
                eprintln!("Error: Unterminated string");
            }
        } else {
            eprintln!("Error: Missing closing quote");
        }
    } else {
        eprintln!("Error: Missing opening quote");
    }
}

// nudhi_die: exit the program
fn nudhi_die() {
    exit(0);
}


// nudhi_set: define variables and handle math with variables
fn nudhi_set(trimmed_line: &str, variables: &mut HashMap<String, Value>) {
    // Example: var_name = 10 or var_name = num1 + num2
    if let Some((var_name, value)) = trimmed_line.split_once('=') {
        let var_name = var_name.trim();
        let value = value.trim();

        // Try to evaluate as integer directly or handle math
        var_name.trim(); 
        if let Ok(int_val) = value.parse::<i32>() {
            variables.insert(var_name.to_string(), Value::Int(int_val));
        } else if let Some(result) = evaluate_math(value, variables) {
            variables.insert(var_name.to_string(), Value::Int(result));
        } else {
            // Otherwise, treat as a string if not a valid math expression
            variables.insert(var_name.to_string(), Value::Str(value.to_string()));
        }
    } else {
        eprintln!("Error: Invalid variable assignment");
    }
}

// Main interpreter function
fn interpret(source_code: &str, variables: &mut HashMap<String, Value>) {
    let lines: Vec<&str> = source_code.lines().collect();

    for line in lines {
        let trimmed_line = line.trim();

        // Skip empty lines
        if trimmed_line.is_empty() {
            continue;
        }

        // Handle nudhi commands
        if trimmed_line.starts_with("nudhi_ask") {
            nudhi_ask(trimmed_line, variables);
        } else if trimmed_line.starts_with("nudhi_change_case") {
            strings::nudhi_change_case(trimmed_line, variables);
        } else if trimmed_line.starts_with("nudhi_do") {
            nudhi_do(trimmed_line);
        } else if trimmed_line.starts_with("nudhi_die") {
            nudhi_die();
        } else if trimmed_line.starts_with("nudhi_read") {
            files::nudhi_read(trimmed_line, variables);
        } else if trimmed_line.starts_with("nudhi_say") {
            nudhi_say(trimmed_line, variables);
        } else if trimmed_line.starts_with("nudhi_write") {
            files::nudhi_write(trimmed_line, variables);
        }
         else if trimmed_line.contains('=') || trimmed_line.contains('+') || trimmed_line.contains('-') || trimmed_line.contains('*') || trimmed_line.contains('/') {
            // Handle variable assignment or math expressions
            nudhi_set(trimmed_line, variables);
        } else {
            eprintln!("nudhi does not know that: {}", line);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        return;
    }
    let source_file = &args[1];

    let source_code = fs::read_to_string(source_file).expect("Failed to read the source file");

    // Initialize variables hashmap
    let mut variables: HashMap<String, Value> = HashMap::new();

    interpret(&source_code, &mut variables);
}
