use std::fs;

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");
    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            //Separated text is allocated inside the heap
            results.push(line.to_string()); //Text is copied not moved here
        }
    }
    results
}

fn main() {
    let text = fs::read_to_string("logs.txt").expect("Failed to read logs.txt");

    let error_logs = extract_errors(text.as_str());

    fs::write("errors2.txt", error_logs.join("\n")).expect("Failed to write errors2.txt");

    // let mut error_logs = vec![];
    // match fs::read_to_string("logs.txt") {
    //     Ok(text) => {
    //         error_logs = extract_errors(text.as_str());
    //         //text will go out of scope

    //         //Writing error logs
    //         match fs::write("errors.txt", error_logs.join("\n")) {
    //             Ok(..) => println!("Wrote errors.txt"),
    //             Err(reason_write_failed) => {
    //                 println!("Writing of errors.txt failed: {}", reason_write_failed)
    //             }
    //         }
    //     }
    //     Err(why_this_failed) => {
    //         println!("Failed to read the file: {}", why_this_failed);
    //     }
    // }
    // println!("{:#?}", error_logs);
}
