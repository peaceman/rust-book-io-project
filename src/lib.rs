mod config;

use std::fs;
use std::error::Error;

pub use config::Config;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};
    use std::fs::File;
    use std::io::Write;
    use std::env;
    use rand::{thread_rng, Rng};

    struct TestFile {
        file_path: Box<Path>,
    }

    impl TestFile {
        fn new(content: &str) -> Result<TestFile, Box<dyn Error>> {
            let file_path = TestFile::gen_file_path();
            let mut file = File::create(&file_path)?;
            file.write_all(content.as_bytes())?;

            Ok(TestFile {
                file_path: file_path.into_boxed_path(),
            })
        }

        fn gen_file_path() -> PathBuf {
            let filename: String = thread_rng()
                .sample_iter(rand::distributions::Alphanumeric)
                .take(8)
                .collect();

            let mut path = env::temp_dir();
            path.set_file_name(filename);

            path
        }
    }

    impl Drop for TestFile {
        fn drop(&mut self) {
            if let Err(e) = fs::remove_file(&self.file_path) {
                eprintln!("Failed to remove tmp file: {}\n{}", self.file_path.to_string_lossy(), e);
            }
        }
    }

    #[test]
    fn run_returns_error_with_non_existing_file() {
        let config = Config { query: "".to_string(), filename: "does-not-exist.txt".to_string(), ..Default::default() };
        let result = run(config);
        assert!(result.is_err());
    }

    #[test]
    fn run_returns_ok_with_existing_file() {
        let test_file = TestFile::new("foobar").unwrap();
        let config = Config {
            query: "".to_string(),
            filename: test_file.file_path.to_str().unwrap().to_string(),
            ..Default::default()
        };

        let result = run(config);
        assert!(result.is_ok());
    }

    #[test]
    fn search_test_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn search_test_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines()
        .filter(|&l| l.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let lc_query = query.to_lowercase();

    content.lines()
        .filter(|&l| l.to_lowercase().contains(&lc_query))
        .collect()
}
