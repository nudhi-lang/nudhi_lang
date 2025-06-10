use std::collections::HashMap;
use crate::Value;

// nudhi_change_case: change the case of a string variable
pub fn nudhi_change_case(trimmed_line: &str, variables: &mut HashMap<String, Value>) {
    let parts: Vec<&str> = trimmed_line.split_whitespace().collect();

    // If the line has three parts then change the case of the variable
    // Example: nudhi_change_case var_name upper
    if parts.len() != 3 || parts[0] != "nudhi_change_case" {
        eprintln!("Error: Invalid syntax for nudhi_change_case");
        return;
    }

    if parts.len() == 3 {
        change_variable_case(parts[1], parts[2], variables);
        return;
    }
}


fn change_variable_case(
    var_name: &str,
    case_type: &str,
    variables: &mut HashMap<String, Value>,
) {
    if let Some(value) = variables.get(var_name) {
        match value {
            Value::Str(val) => {
                let new_val = match case_type {
                    "upper" => val.to_uppercase(),
                    "lower" => val.to_lowercase(),
                    _ => {
                        eprintln!("Error: Invalid case type '{}'", case_type);
                        return;
                    }
                };
                variables.insert(var_name.to_string(), Value::Str(new_val));
            }
            _ => eprintln!("Error: Variable '{}' is not a string", var_name),
        }
    } else {
        eprintln!("Error: Variable '{}' not found", var_name);
    }
}