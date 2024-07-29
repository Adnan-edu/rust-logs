use std::fs;

fn extract_errors(text: &str) -> Vec<&str> {
    let split_text = text.split("\n");
    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line);
        }
    }
    results
}

fn main() {
    let mut error_logs = vec![];
    match fs::read_to_string("logs.txt") {
        Ok(text) => {
            error_logs = extract_errors(text.as_str());
            //text will go out of scope
        }
        Err(why_this_failed) => {
            println!("Failed to read the file: {}", why_this_failed);
        }
    }
    println!("{:#?}", error_logs);
}
