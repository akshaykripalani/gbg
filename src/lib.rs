const LOG_FLAG: &'static str = "-l";


pub struct Config {
    pub will_log: bool,
    pub log_path: String,
    pub command: String,
}

impl Config {
    pub fn build(args: impl Iterator<Item = String>) -> Result<Config, &'static str> {

        let mut args = args.skip(1).peekable();

        let will_log = args.peek().map(|s| s.as_str()) == Some(&LOG_FLAG);
        

        let log_path = if will_log {
            args.next(); // consume the log flag
            match args.next() {
                Some(path) => path,
                None => return Err("Missing log path after -l flag"),
            }
        } else {
            String::new()
        };

        let command: String = args.collect::<Vec<_>>().join(" ");
        
        if command.is_empty() {
            return Err("Missing arguments");
        }

        Ok(Config {
            will_log,
            log_path,
            command })
    }
}