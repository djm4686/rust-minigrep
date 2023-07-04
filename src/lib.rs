use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
        query: String,
        file_path: String,
	case_insensitive: bool,
}

impl Config {
        pub fn build(args: &[String]) -> Result<Config, &'static str> {
                if args.len() < 3 {
                        return Err("Not enough args")
                }
                let query = args[1].clone();
                let file_path = args[2].clone();
		let case_insensitive: bool = match env::var("IGNORE_CASE"){
			Ok(x) => x == String::from("true"),
			_ => false,
		};
                Ok(Config {query, file_path, case_insensitive })
        }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_path)?;
	let mut results;
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
	let mut results = Vec::new();
	for line in contents.lines() {
		if line.contains(query){
			results.push(line);
		}	
	}
	results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	let mut results = Vec::new();
	for line in contents.lines() {
		if line.to_lowercase().contains(&query){
			results.push(line);
		}
	}
	results

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
