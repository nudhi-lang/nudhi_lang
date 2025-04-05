// funtinalty to read and write files
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::collections::HashMap;
use crate::Value;

pub fn nudhi_read(trimmed_line: &str, variables: &mut HashMap<String, Value>) {
    let parts: Vec<&str> = trimmed_line.split_whitespace().collect();
    
    if parts.len() != 3 {
        eprintln!("Error: Syntax is: nudhi_read \"filename.txt\" variable_name");
        return;
    }

    if let Some(start) = parts[1].find('"') {
        if let Some(end) = parts[1].rfind('"') {
            if start != end {
                let filename = &parts[1][start + 1..end];
                let var_name = parts[2];

                match fs::read_to_string(filename) {
                    Ok(content) => {
                        variables.insert(var_name.to_string(), Value::Str(content));
                    },
                    Err(e) => eprintln!("Error reading file '{}': {}", filename, e)
                }
            } else {
                eprintln!("Error: Unterminated string in filename");
            }
        } else {
            eprintln!("Error: Missing closing quote in filename");
        }
    } else {
        eprintln!("Error: Filename must be in quotes");
    }
}

pub fn nudhi_write(trimmed_line: &str, variables: &HashMap<String, Value>) {
    let parts: Vec<&str> = trimmed_line.split_whitespace().collect();
    
    if parts.len() < 3 {
        eprintln!("Error: Syntax is: nudhi_write \"filename.txt\" \"content\" or nudhi_write \"filename.txt\" variable_name");
        return;
    }

    // Extract filename
    if let Some(start) = parts[1].find('"') {
        if let Some(end) = parts[1].rfind('"') {
            if start != end {
                let filename = &parts[1][start + 1..end];
                
                // Get content either from quoted string or variable
                let content = if parts[2].starts_with('"') {
                    if let Some(content_end) = parts[2].rfind('"') {
                        Some(parts[2][1..content_end].to_string())
                    } else {
                        eprintln!("Error: Missing closing quote in content");
                        return;
                    }
                } else {
                    // Content is a variable name
                    match variables.get(parts[2]) {
                        Some(Value::Int(val)) => Some(val.to_string()),
                        Some(Value::Str(val)) => Some(val.clone()),
                        None => {
                            eprintln!("Error: Variable '{}' not found", parts[2]);
                            return;
                        }
                    }
                };

                if let Some(content) = content {
                    let mut file = match OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(filename) {
                            Ok(file) => file,
                            Err(e) => {
                                eprintln!("Error opening file '{}' for writing: {}", filename, e);
                                return;
                            }
                        };

                    if let Err(e) = file.write_all(content.as_bytes()) {
                        eprintln!("Error writing to file '{}': {}", filename, e);
                    }
                }
            } else {
                eprintln!("Error: Unterminated string in filename");
            }
        } else {
            eprintln!("Error: Missing closing quote in filename");
        }
    } else {
        eprintln!("Error: Filename must be in quotes");
    }
}