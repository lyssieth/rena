/*
MIT License
Copyright (c) 2020-2023 Lyssieth

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
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
        Err(e) => Err(format!("Failed to parse usize: {}", e)),
    }
}

pub fn validate_regex(input: &str) -> Result<(), String> {
    let a = regex::Regex::new(input);

    match a {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to parse regex: {}", e)),
    }
}
