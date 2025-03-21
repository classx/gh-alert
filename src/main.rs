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

        println!(
            "Changes found in the following repositories: {}",
            changed_repos.join(", ")
        );

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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_deserialization() {
        let config_yaml = r#"
            repositories:
              - owner: test-owner
                repo: test-repo
                token: test-token
                branch: main
                files:
                  - test.txt
            state_file: state.yaml
            notification_command: notify
        "#;
        let config: Config = serde_yaml::from_str(config_yaml).unwrap();
        assert_eq!(config.repositories[0].owner, "test-owner");
        assert_eq!(config.repositories[0].repo, "test-repo");
        assert_eq!(config.repositories[0].token, "test-token");
        assert_eq!(config.repositories[0].branch, "main");
        assert_eq!(config.repositories[0].files[0], "test.txt");
        assert_eq!(config.state_file, "state.yaml");
        assert_eq!(config.notification_command, "notify");
    }

    #[test]
    fn test_state_serialization() {
        let mut files = HashMap::new();
        files.insert("owner/repo/file.txt".to_string(), "hash123".to_string());
        let state = State { files };
        let serialized = serde_yaml::to_string(&state).unwrap();
        let deserialized: State = serde_yaml::from_str(&serialized).unwrap();
        assert_eq!(
            deserialized.files.get("owner/repo/file.txt").unwrap(),
            "hash123"
        );
    }

    #[test]
    fn test_get_file_hash() {
        let client = Client::new();
        let url = "https://raw.githubusercontent.com/rust-lang/rust/refs/heads/master/README.md";
        let hash = get_file_hash(&client, url).unwrap();
        println!("SHA: {}", hash);
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_check_for_changes() {
        let temp_file = NamedTempFile::new().unwrap();
        let state_path = temp_file.path().to_str().unwrap();

        let config = Config {
            repositories: vec![Repository {
                owner: "rust-lang".to_string(),
                repo: "rust".to_string(),
                token: "".to_string(),
                branch: "master".to_string(),
                files: vec!["README.md".to_string()],
            }],
            state_file: state_path.to_string(),
            notification_command: "echo 'changed'".to_string(),
        };

        // First run should detect changes
        let _ = check_for_changes(&config);

        // Second run should not detect changes
        let _ = check_for_changes(&config);
    }
}
