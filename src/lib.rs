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
    
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let query = &query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(query) {
            results.push(line);
        }
    }

    results
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_config() {
        let args = &["minigrep".to_string(), "query".to_string(), "file".to_string()];
        let config = Config::new(args);

        assert!(config.is_ok());
        let config = config.unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "file");
        assert_eq!(config.case_sensitive, true);
    }

    #[test]
    fn new_config_4_args() {
        let args = &["minigrep".to_string(), "query".to_string(), "file".to_string(), "to-much".to_string()];
        let config = Config::new(args);

        assert!(config.is_ok());
        let config = config.unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "file");
        assert_eq!(config.case_sensitive, true);
    }

    #[test]
    fn new_config_2_args() {
        let args = &["minigrep".to_string(), "query".to_string()];
        let config = Config::new(args);

        assert!(config.is_err());
    }

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
