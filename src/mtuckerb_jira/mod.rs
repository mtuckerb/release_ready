use crate::set_redis;
use crate::MtuckerbConfig as MtuckerbConfig;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub expand: String,
    pub id: String,
    pub key: String,
    pub fields: Fields,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fields {
    pub statuscategorychangedate: String,
    pub issuetype: IssueType,
    pub sprint: Sprint,
    pub workratio: f64,
    pub created: String,
    pub labels: Vec<String>,
    pub updated: String,
    pub description: String,
    pub flagged: bool,
    pub summary: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueType {
    pub id: String,
    pub description: String,
    pub name: String,
    pub subtask: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sprint {
    pub id: usize,
    pub state: String,
    pub name: String,
}

pub async fn lookup_issue(
    message_id: &str,
    auth_token: &str,
    config: &MtuckerbConfig,
) -> Result<Issue, String> {
    let issue = match reqwest::Client::new()
    .get(format!(
        "https://{}.atlassian.net/rest/agile/1.0/issue/{}",
        config.subdomain, message_id
    ))
    .header("Content-Type", "application/json")
    .header("Authorization", format!("Basic {}", &auth_token))
    .send()
    .await {
        Ok(resp) => { 
            match resp.json::<Issue>().await {
                Ok(val) => Ok(val),
                Err(err) => Err(format!("Could not find issue: {} in {}: {}", message_id, config.subdomain, err.to_string())),
            }
        },
        Err(e) => panic!("Could not connect to Jira: {}", e),
    };
   
    return match issue {
        Ok(val) => Ok(val),
        Err(val) => Err(val),
    };
}

