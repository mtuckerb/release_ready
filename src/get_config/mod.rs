pub mod get_config {
    use confy;
    use futures_util::StreamExt;
    use serde_derive::{Deserialize, Serialize};
    use tokio::io;
    use tokio_util::codec::{FramedRead, LinesCodec};
    use regex::Regex;

    #[derive(Default, Debug, Serialize, Deserialize)]
    pub struct MtuckerbConfig {
        pub jira_email: String,
        pub jira_password: String,
        pub board_id: String,
        pub subdomain: String,
    }

    pub async fn get_config() -> MtuckerbConfig {
        let config: MtuckerbConfig = match confy::load("mtuckerb", "release_ready") {
            Ok(cfg) => cfg,
            Err(_) => {
                return set_config().await;
            }
        };

        if !(config.jira_email == "")
            || !(config.jira_password == "")
            || !(config.board_id == "")
            || !(config.subdomain == "")
        {
            return config;
        }

        set_config().await
    }


    pub async fn set_config() -> MtuckerbConfig {
        let mut config: MtuckerbConfig =  confy::load("mtuckerb", "release_ready").unwrap();
        let file = confy::get_configuration_file_path("mtuckerb", "release_ready").unwrap();
        println!("Configuration file path is: {:#?}", file);

        let stdin = io::stdin();
        let mut reader = FramedRead::new(stdin, LinesCodec::new());
        if config.jira_email == "" {
            println!("\nPlease enter your Jira email and hit <cr>:");
            config.jira_email = reader.next().await.transpose().unwrap().unwrap();
        }
        if config.jira_password == "" {
            println!("\nNow please enter the API token that you created at\n https://id.atlassian.com/manage-profile/security/api-tokens\n and hit <cr>:");
            config.jira_password = reader.next().await.transpose().unwrap().unwrap();
        }
        if config.board_id == "" {
            println!("\nOkay! please enter the board id that you want to check\n");
            config.board_id = reader.next().await.transpose().unwrap().unwrap();
        }
        if config.subdomain == "" {
            println!("\nLastly, please enter the subdomain for your Atlassian cloud");
            config.subdomain = reader.next().await.transpose().unwrap().unwrap();
        }
        confy::store("mtuckerb", "release_ready", &config)
            .expect("Failed to store configuration");

        
        match std::env::current_exe() {
            Ok(exe_path) => {
                let path_string = exe_path.display().to_string();
                let re = Regex::new(r"^/usr/local/bin/").unwrap();
                if !re.is_match(&path_string){
                    println!("Your executable is in {}. Would you like to move it to /usr/local/bin? (Y/N)", &path_string);
                    if reader.next().await.transpose().unwrap().unwrap().to_lowercase() == "y" {
                        match crate::fs::copy(&path_string, "/usr/local/bin/release_ready") {
                            Ok(_) => {println!("Moved the executable to /usr/local/bin/release_ready")},
                            Err(e) => {println!("{}", e)}
                        }
                    };
                };
            
            },
            Err(e) => println!("{}",e),
        }
        return config;
    }    
}
