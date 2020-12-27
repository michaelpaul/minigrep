use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // skip program name
        args.next();
        let query = match args.next() {
            Some(v) => v,
            None => return Err("missing query"),
        };
        let filename = match args.next() {
            Some(v) => v,
            None => return Err("missing filename"),
        };

        Ok(Config {
            query,
            filename,
            case_sensitive: env::var("INSENSITIVE").is_err(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        isearch(&config.query, &contents)
    };
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

pub fn isearch<'b>(query: &str, contents: &'b str) -> Vec<&'b str> {
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

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn no_result() {
        let res: Vec<&str> = Vec::new();
        assert_eq!(res, search("foo", "bar"));
    }

    #[test]
    fn multiple_results() {
        let contents = "foo
nobody
    somebody
<body>delete</body>
bar";
        assert_eq!(
            vec!["nobody", "    somebody", "<body>delete</body>"],
            search("body", contents)
        );
    }

    #[test]
    fn case_insensitive() {
        assert_eq!(
            vec!["Trust", " in RUST!"],
            isearch("RuSt", "Trust\nme\nI'm written\n in RUST!")
        );
    }

    #[test]
    fn case_insensitive_no_match() {
        assert_eq!(Vec::<&str>::new(), isearch("Neo", "Final\nFantasy"));
    }
}
