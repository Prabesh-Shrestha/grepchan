use std::env;
use std::error::Error;
use std::fs;

pub struct Args<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub case_sensitive: bool,
}

impl Args<'_> {
    pub fn new(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 3 {
            return Err("Too few arguments");
        }
        let query = &args[1];
        let filename = &args[2];
        println!("Searching for \"{}\" \nInside {}", query, filename);
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Args {
            query,
            filename,
            case_sensitive,
        })
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

pub fn grep_file(args: Args) -> Result<(), Box<dyn Error>> {
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

// pub fn grep_dir(args: Args) -> Result<(), Box<dyn Error>> {

//     Ok(())
// }

// complete this
// returns the paths of every files inside the folder
pub fn read_dir(path_file: &str) -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();
    let mut dirs: Vec<String> = vec![path_file.to_string()];
    
    vec![String::new()]
}

// checks if a path is to a dir or a file 
pub fn is_dir(path_file: &str) -> bool {
    fs::metadata(path_file).unwrap().is_dir()
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
        assert_eq!(
            vec!["safe, fast, produCtive."],
            search_case_insensetive(query, contents)
        );
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
