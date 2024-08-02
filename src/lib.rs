use std::error::Error;
use std::fs;
use std::env;


pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("無法取得搜尋字串"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("無法取得檔案路徑")
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        return Ok(Config { 
            query: query, 
            file_path: file_path,
            ignore_case: ignore_case, 
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case{
        search(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };

    for line in results{
        println!("{}", line);
    }

    Ok(())
}

// 因為 contents 包含所有文字且我們想要回傳的部分文字，所以我們將 contents 與回傳值參考做關聯 (白話：我們只需要 contents！)
// 告訴 rust 回傳值參考會活得跟 contents 一樣久
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents
    .lines()
    .filter(|line| line.to_lowercase().contains(query))
    .collect()
}

#[cfg(test)]
mod tests{
    // use crate::search; // 等同於 super::search
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}