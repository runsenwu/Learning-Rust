use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(input: &[String]) -> Result<Config, &'static str> {
        if input.len() != 3 {
            return Err("Incorrect amount of input elements");
        }

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query: input[1].clone(),
            file_path: input[2].clone(),
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut ret: Vec<&'a str> = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            ret.push(line);
        }
    }

    ret
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut ret: Vec<&'a str> = Vec::new();
    let lower_query = query.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(&lower_query) {
            ret.push(line);
        }
    }

    ret
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &file_content)
    } else {
        search(&config.query, &file_content)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive_search() {
        let query = "DUCT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_insensitive(query, contents)
        );
    }
}
