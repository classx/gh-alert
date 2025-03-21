use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_yaml;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::{self, File};
//use std::io::{self, Read, Write};

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    repositories: Vec<Repository>,
    state_file: String,
    notification_command: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Repository {
    owner: String,
    repo: String,
    token: String,
    branch: String,
    files: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct State {
    files: HashMap<String, String>,
}

fn get_file_hash(client: &Client, url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = client.get(url).send()?;
    let content = response.text()?;
    let mut hasher = Sha256::new();
    hasher.update(content);
    let hash = format!("{:x}", hasher.finalize());
    Ok(hash)
}

fn get_current_state(config: &Config) -> Result<State, Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut files = HashMap::new();
    for repo in &config.repositories {
        for file in &repo.files {
            let url = format!(
                "https://raw.githubusercontent.com/{}/{}/refs/heads/{}/{}?token={}",
                repo.owner, repo.repo, repo.branch, file, repo.token
            );
            let hash = get_file_hash(&client, &url)?;
            files.insert(format!("{}/{}/{}", repo.owner, repo.repo, file), hash);
        }
    }
    Ok(State { files })
}

fn check_for_changes(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let current_state = get_current_state(config)?;
    let previous_state: State = match fs::read_to_string(&config.state_file) {
        Ok(s) => serde_yaml::from_str(&s)?,
        Err(_) => State {
            files: HashMap::new(),
        },
    };
    if current_state.files != previous_state.files {
        let mut changed_repos = Vec::new();
        for (file_key, current_hash) in &current_state.files {
            if let Some(previous_hash) = previous_state.files.get(file_key) {
                if current_hash != previous_hash {
                    let parts: Vec<&str> = file_key.split('/').collect();
                    if parts.len() >= 2 {
                        let repo_name = format!("{}/{}", parts[0], parts[1]);
                        if !changed_repos.contains(&repo_name) {
                            changed_repos.push(repo_name);
                        }
                    }
                }
            } else {
                let parts: Vec<&str> = file_key.split('/').collect();
                if parts.len() >= 2 {
                    let repo_name = format!("{}/{}", parts[0], parts[1]);
                    if !changed_repos.contains(&repo_name) {
                        changed_repos.push(repo_name);
                    }
                }
            }
        }

        println!("Changes found in the following repositories: {}",
            changed_repos.join(", "));

        let state_file = File::create(&config.state_file)?;
        serde_yaml::to_writer(state_file, &current_state)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_file = fs::read_to_string("config.yaml")?;
    let config: Config = serde_yaml::from_str(&config_file)?;

    check_for_changes(&config)?;

    Ok(())
}
