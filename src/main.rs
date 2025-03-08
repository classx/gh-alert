use clap::{Parser, Subcommand};
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
// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Args {
//     /// Config file path
//     #[arg(short = 'f', long = "file")]
//     config: String,
//     /// Validate config file only
//     #[arg(short, long, default_value = "false")]
//     test: bool,
//     /// Print debug information
//     #[arg(short = 'd', long = "debug", default_value = "false")]
//     debug: bool,
// }

/// CLI tool with subcommands
#[derive(Parser, Debug)]
#[command(
    name = "Github monitoring tool",
    version = "1.0",
    about = "A tool with subcommands"
)]
struct Cli {
    /// Subcommands for specific actions
    #[command(subcommand)]
    command: Commands,
}

/// Define available subcommands
#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new item
    Init {
        /// Name of the item to add
        //#[arg(short = 'f', long = "file", default_value = "")]
        name: String,
    },
    /// Remove an item by its ID
    Remove {
        /// ID of the item to remove
        id: u32,
    },
}

// struct functions
// define default values for bool fields
fn default_bool() -> bool {
    false
}

impl Repo {
    fn run(&self) {
        println!("Repo run {}", &self.name);
    }
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

fn generate_config_yaml(config: &Config) -> std::io::Result<()> {
    let yaml_str = serde_yaml::to_string(&config).unwrap();
    std::fs::write("config.yaml", yaml_str)?;
    Ok(())
}

// fn main() {
//     let args = Args::parse();
//     let config = load_config(&args.config);
//     if args.test {
//         validation(&config);
//         println!("Config file is valid.");
//         return;
//     }
//     if args.debug {
//         println!("{:?}", config);
//     }

//     for repo in config.repos {
//         if repo.skip {
//             continue;
//         }
//         repo.run();
//     }
// }

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init { name } => {
            let config = Config {
                project: name.to_string(),
                path: "/dir".to_string(),
                update_frequency_sec: 3600,
                message: vec![Message {
                    name: "name".to_string(),
                    url: "url".to_string(),
                }],
                repos: vec![Repo {
                    name: "repo name".to_string(),
                    url: "url".to_string(),
                    branch: "main".to_string(),
                    skip: false,
                    silent: false,
                    files: vec!["file".to_string()],
                }],
            };
            let _ = generate_config_yaml(&config);
        }
        Commands::Remove { id } => {
            println!("Removing item with ID: {}", id);
        }
    }
}
