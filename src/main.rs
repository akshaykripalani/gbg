use std::{env, process};
use std::process::Command;
use gbg::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    let nohup_command:String;
    let output_message: String;

    if config.will_log {
        nohup_command = format!("nohup {} > {} 2>&1 &", config.command, config.log_path);
        output_message = format!("Logging to {}", config.log_path);
    }
    else {
        nohup_command = format!("nohup {} > /dev/null 2>&1 &", config.command);
        output_message = format!("Skipping Logging");
    }

    
    Command::new("sh")
        .arg("-c")
        .arg(&nohup_command)
        .spawn()
        .unwrap_or_else(|err| {
            eprintln!("Failed to execute command: {}", err);
            process::exit(1);
        });

    println!("{}...", output_message);
    println!("Command '{}' started in background", config.command);

}