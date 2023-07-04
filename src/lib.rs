use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
        query: String,
        file_path: String,
	case_insensitive: bool,
}

impl Config {
        pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
		args.next();
		let query = match args.next() {
			Some(arg) => arg,
			None => return Err("Didnt get a query string")
		};
		let file_path = match args.next(){
			Some(arg) => arg,
			None => return Err("Didnt get a file path"),
		};
		let case_insensitive: bool = match env::var("IGNORE_CASE"){
			Ok(x) => x == String::from("true"),
			_ => false,
		};
                Ok(Config {query, file_path, case_insensitive })
        }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_path)?;
	let results;
	if config.case_insensitive{
		results = search_case_insensitive(&config.query, &contents);
	}
	else{
		results = search(&config.query, &contents);
        }
	for line in results{
            println!("{}", line);
        }
        Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
	contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	contents.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
}

#[cfg(test)]
mod tests{
	use super::*;
	
	#[test]
	fn one_result(){
		let query = "duct";
		let contents = "\
Ruse:
safe, fast, productive.
Pick three.
Duct tape";
		
		assert_eq!(
			vec!["safe, fast, productive."],
			search(query, contents)
		);
	}
	#[test]
	fn case_insensitive(){
		let query = "RuSt";
		let contents = "\
Rust:
safe, fast, productive.
pick three.";
		
		assert_eq!(
			vec!["Rust:"],
			search_case_insensitive(query, contents)
		);
	}
}
