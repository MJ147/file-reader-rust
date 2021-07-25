use std::env;
use std::process;
mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = lib::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_name);

    if let Err(e) = lib::run(config) {
        println!("Application error: {}", e);
        process::exit(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "Sec";
        let contents = "\
First test line
Second test line
Third test line";

        assert_eq!(vec!["Second test line"], lib::search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "Test";
        let contents = "\
First test line
Second test line
Third test line";

        assert_eq!(vec!["First test line", "Second test line", "Third test line"], lib::search_case_insensitive(query, contents));
    }
}
