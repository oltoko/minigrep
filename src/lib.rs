use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let file_content = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &file_content) {
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

pub struct Config {
    query: String,
    filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename})
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
    }

    #[test]
    fn new_config_4_args() {
        let args = &["minigrep".to_string(), "query".to_string(), "file".to_string(), "to-much".to_string()];
        let config = Config::new(args);

        assert!(config.is_ok());
        let config = config.unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "file");
    }

    #[test]
    fn new_config_2_args() {
        let args = &["minigrep".to_string(), "query".to_string()];
        let config = Config::new(args);

        assert!(config.is_err());
    }

    #[test]
    fn one_result() {
        
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three!";

        assert_eq!(vec!["safe, fast, productive."], search(query, content))
    }
}
