use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let file_content = fs::read_to_string(config.filename)?;

    let results;
    
    if config.case_sensitive {
        results = search(&config.query, &file_content);
    } else {
        results = search_case_insensitive(&config.query, &file_content);
    }

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let query = &query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name")
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case_sensitive() {
        
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content))
    }

    #[test]
    fn case_insensitive() {
        
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, content))
    }
}
