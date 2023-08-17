
pub mod get_config;
pub use get_config::get_config::get_config as get_config;
pub use get_config::get_config::set_config as set_config;

use get_config::get_config::MtuckerbConfig;

pub mod mtuckerb_jira ;
pub use mtuckerb_jira::lookup_issue as lookup_issue;

pub mod mtuckerb_redis;
pub use mtuckerb_redis::check_redis as check_redis;
pub use mtuckerb_redis::set_redis as set_redis;

use std::process::Command;
//use colored::Colorize;
use clap::Parser;
use base64::{engine::general_purpose, Engine as _};
use colored::Colorize;
use regex::Regex;
pub use std::fs;

#[derive(Parser)]
struct Cli {
    #[structopt(name = "config", long = "config")]
    config: bool,
 
}

async fn run(args: &Cli) -> Result<(), String>  {
    let config: MtuckerbConfig = get_config().await;
    
    if args.config {
        println!("launching config");
        set_config().await;
    }
    
    let auth_token = general_purpose::STANDARD_NO_PAD
        .encode(format!("{}:{}", &config.jira_email, &config.jira_password));

    let log = git_log();
 
    for x in log.unwrap().iter() {
        let re = Regex::new(r"^(\w+\/)?(?P<issue_no>\w+[-\s]\d+) ").unwrap();
        let message_id = match re.captures(x) {
            Some(m) => match m.name("issue_no") {
                Some(mes) => mes.as_str(),
                None => {
                    return Err(format!(
                        "{}",
                        "Your commit does not appear to start with an Issue"
                            .red()
                            .bold()
                    ));
                }
            },
            None => {
                return Err(format!( "{} does not have a Issue #", x.bold().red()));
            }
        };
        match mtuckerb_jira::lookup_issue(message_id, &auth_token, &config).await {
            Ok(response) => { 
                let bold_message = format!("{}", x.bold().green());
                println!("{}: {}", bold_message, response.fields.
            labels.join(",").bold().yellow()); 
        },
            Err(error) => { format!("{}", error).red().bold(); }
        }

    }

    Ok(())
}


fn git_log() -> Result<Vec<String>, String> { 
    let cmd = Command::new("sh")
        .arg("-c")
        .arg(format!("git log --since 2023-06-15  --no-merges --pretty=format:'%s  %h' main release "))
        .output();

    match cmd {
        Ok(output) => {
            let cmd_string = String::from_utf8_lossy(&output.stdout);
            let lines = cmd_string.split("\n");
            let response = lines.map(ToString::to_string).collect();
            Ok(response)
        }
        Err(error) => Err(format!("Failed to execute git log: {}", error))
    }
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match run(&args).await {
        Ok(()) => {}
        Err(e) => println!("{}", e),
    }
}