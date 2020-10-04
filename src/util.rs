use std::path::Path;

pub fn validate_directory(input: &str) -> Result<(), String> {
    let path = Path::new(input);

    if path.exists() && path.is_dir() {
        Ok(())
    } else {
        Err(format!("Invalid directory: {}", input))
    }
}

pub fn validate_usize(input: &str) -> Result<(), String> {
    let num = input.parse::<usize>();

    match num {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to parse usize: {}", e.to_string())),
    }
}

pub fn validate_regex(input: &str) -> Result<(), String> {
    let a = regex::Regex::new(input);

    match a {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to parse regex: {}", e)),
    }
}
