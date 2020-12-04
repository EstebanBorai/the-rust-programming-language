use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Self { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &file_contents) {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_config_instance() {
        let config = Config::new(&vec![
            "executable_path".to_string(),
            "search".to_string(),
            "foo.txt".to_string(),
        ]);

        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.query, "search".to_string());
        assert_eq!(config.filename, "foo.txt".to_string());
    }

    #[test]
    fn validates_args_for_config_instance() {
        let config = Config::new(&vec![
            "search".to_string(),
            "foo.txt".to_string(),
        ]);

        assert!(config.is_err());

        if let Err(message) = config {
            assert_eq!(message, "not enough arguments")
        }
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
