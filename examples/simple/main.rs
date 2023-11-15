use log_file_analyser::match_strings;

fn main() {
    match match_strings(vec![
        "line number 2",
        "this is something else"
    ], "./test.log") {
        Ok(res) => {
            println!("Success!");
            println!("{}", res);
        },
        Err(e) => println!("Error: {:?}", e),
    }
}
