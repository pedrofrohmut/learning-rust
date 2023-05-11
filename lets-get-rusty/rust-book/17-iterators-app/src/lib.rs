use std::error::Error;
use std::{fs, env};

pub fn run(config: Config) -> Result<(), Box<dyn Error>>
{
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.ignore_case {
        search_ignore_case(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results { println!("{}", line); }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str>
    {
        let _skip_me = args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get the query string")
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get filename")
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, filename, ignore_case })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
{
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_ignore_case<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
{
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_uppercase().contains(&query.to_uppercase()) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result()
    {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive()
    {
        let query = "rUST";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."],
                   search_ignore_case(query, contents));
    }
}
