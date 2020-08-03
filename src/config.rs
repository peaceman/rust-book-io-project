use std::env;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_rejects_less_than_3_arguments() {
        let config = Config::new(&["foo".to_string()]);
        assert!(!config.is_ok());
    }

    #[test]
    fn config_accepts_3_arguments() {
        let config = Config::new(&[
            "executable".to_string(),
            "query".to_string(),
            "filename".to_string()
        ]);
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!("query", config.query);
        assert_eq!("filename", config.filename);
    }
}

#[derive(Debug)]
#[derive(Default)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}
