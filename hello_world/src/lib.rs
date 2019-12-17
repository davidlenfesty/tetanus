use std::fs;
use std::env;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {query, filename, case_sensitive})
    }
}

/// Runs the grep searcher thingie
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    return Ok(());
}

// This says that the lifetime of the contents string and the
// returned value are the same
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str)
    -> Vec<&'a str> {
        let query = query.to_lowercase();
        let mut results = Vec::new();

        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line);
            }
        }

        results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::ErrorKind;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents)
    );
    }

    #[test]
    fn config_too_few_args() {
        let args = [String::from("foo")];
        
        assert_eq!(Config::new(&args), Err("not enough arguments"));
    }

    #[test]
    fn config_no_args() {
        let args = [];
        
        assert_eq!(Config::new(&args), Err("not enough arguments"));
    }

    #[test]
    fn run_file_does_not_exist() {
        let config = Config {
            query: String::from("foo"),
            filename: String::from("bar.txt"),
            case_sensitive: false
        };

        let error = run(config)
            .expect_err("run should error if a file does not exist")
            .downcast_ref::<std::io::Error>().unwrap().kind();

        assert_eq!(error, ErrorKind::NotFound);

    }
}