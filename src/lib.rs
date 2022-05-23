use std::error::Error;
use std::fs;
use std::env;

pub struct Args {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}
impl Args {
    pub fn new(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 3 {
            return Err("Too few arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        println!("Searching for \"{}\" \nInside {}", query, filename);
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Args { query, filename, case_sensitive})
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::<&str>::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensetive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::<&str>::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            result.push(line);
        }
    }
    result
}

pub fn grep(args: Args) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&args.filename)?;
    // .expect(format!("Cannot open the file {}", args.filename).as_str());
    let res = if args.case_sensitive {
        search(&args.query, &content)
    } else {
        search_case_insensetive(&args.query, &content)
    };

    println!("found: ");
    for line in res {
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_insensetive_test() {
        let query = "dUct";
        let contents = "\
Rust:
safe, fast, produCtive.
Pick three.";
        assert_eq!(vec!["safe, fast, produCtive."], search_case_insensetive(query, contents));
    }

    #[test]
    fn case_sensetive_test() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
