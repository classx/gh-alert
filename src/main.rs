use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use tabled::{Table, Tabled};

// Structs

#[derive(Tabled)]
struct Yamlstatus<'a> {
    name: &'a str,
    value: &'a str,
    status: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    project: String,
    path: String,
    update_frequency_sec: u32,
    message: Vec<Message>,
    repos: Vec<Repo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    name: String,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Repo {
    name: String,
    url: String,
    branch: String,
    #[serde(default = "default_bool")]
    skip: bool,
    #[serde(default = "default_bool")]
    silent: bool,
    files: Vec<String>,
}

/// Github monitoring tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Config file path
    #[arg(short = 'f', long = "file")]
    config: String,
    /// Validate config file only
    #[arg(short, long, default_value = "false")]
    test: bool,
}

// struct functions
// define default values for bool fields
fn default_bool() -> bool {
    false
}

// Functions

fn load_config(file: &str) -> Config {
    let f = std::fs::File::open(file).expect("Could not open file.");
    let scrape_config: Config = serde_yaml::from_reader(f).expect("Could not read values.");
    scrape_config
}

fn validation(config: &Config) {
    let statuses = vec![
        Yamlstatus {
            name: "Project",
            value: &config.project,
            status: "OK",
        },
        Yamlstatus {
            name: "Project path",
            value: &config.path,
            status: "OK",
        },
    ];
    let table = Table::new(statuses);
    println!("{table}");
}

fn main() {
    let args = Args::parse();
    let config = load_config(&args.config);
    if args.test {
        validation(&config);
        println!("Config file is valid.");
        return;
    }
    println!("{:?}", config);
}
