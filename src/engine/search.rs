use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use structopt::StructOpt;

use crate::logger::{LogExtraParams, Logger};

#[derive(StructOpt)]
#[structopt(name = "grepper")]
pub struct Config {
    /// the pattern to look for in a file
    pub lookup_pattern: String,
    /// path or paths of file(s) to search in
    #[structopt(parse(from_os_str), multiple = true)]
    pub file_paths: Vec<std::path::PathBuf>,
    /// enable debug mode
    #[structopt(short = "d", long = "debug")]
    pub debug: bool,
}

pub fn search<'a>(query: &str, content: &'a str) -> Option<&'a str> {
    if content.contains(query) {
        Some(content)
    } else {
        None
    }
}

pub fn run_search(config: Config, logger: Logger) -> Result<(), Box<dyn Error>> {
    for file_path in config.file_paths {
        let mut log_params = LogExtraParams::new();
        log_params.insert("file", file_path.to_str().unwrap());

        logger.debug("Starting search", Some(&log_params));
        let fp = File::open(file_path)?;
        let reader = BufReader::new(fp);

        for line_result in reader.lines() {
            let line = line_result.unwrap();
            match search(&config.lookup_pattern, &line) {
                Some(matching_line) => println!("{}", matching_line),
                None => continue,
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct".to_string();
        let content = "safe, fast, productive.".to_string();
        let search_result = search(&query, &content);

        assert_eq!("safe, fast, productive.", search_result.unwrap(),);
    }
}
