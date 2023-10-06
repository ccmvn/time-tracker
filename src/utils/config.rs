use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use lazy_static::lazy_static;
use serde_derive::Deserialize;

// Define the configuration file name
const CONFIG_FILE_NAME: &str = "config.toml";

// Function to generate the list of possible configuration file paths
fn generate_config_file_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    #[cfg(target_os = "macos")]
    {
        paths.push(PathBuf::from("./"));
        paths.push(PathBuf::from("../"));
        paths.push(PathBuf::from("../../"));
    }

    #[cfg(target_os = "windows")]
    {
        paths.push(PathBuf::from(".\\"));
        paths.push(PathBuf::from("..\\"));
        paths.push(PathBuf::from("..\\..\\"));
    }

    paths.into_iter().map(|mut path| {
        path.push(CONFIG_FILE_NAME);
        path
    }).collect()
}

// Possible locations for the configuration file
lazy_static! {
    pub static ref FILES: Vec<PathBuf> = generate_config_file_paths();
}

// Function to check if the configuration file exists
pub fn config_exists(paths: &[PathBuf]) -> bool {
    paths.iter().any(|path| Path::new(path).exists())
}

// Struct to hold the configuration
#[derive(Eq, PartialEq, Hash, Clone, Deserialize)]
pub struct GlobalConfig {
    pub session: SessionConfig,
}

// Struct to hold the session configuration
#[derive(Eq, PartialEq, Hash, Clone, Deserialize)]
pub struct SessionConfig {
    pub secret_key: String,
}

pub struct Config {
    config: GlobalConfig,
}

// Function to attempt opening the first available configuration file from a list of paths
fn open_first_available_file(paths: &[PathBuf]) -> std::io::Result<File> {
    let mut errors = Vec::new();

    for path in paths {
        match File::open(&path) {
            Ok(file) => return Ok(file),
            Err(e) => errors.push((path.clone(), e)),
        }
    }

    let error_message = errors
        .into_iter()
        .map(|(path, error)| format!("{}: {}", path.display(), error))
        .collect::<Vec<String>>()
        .join("\n");

    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!("No configuration file found:\n{}", error_message),
    ))
}

impl Config {
    // Constructor for the Config struct. It reads the configuration file and creates an instance of the struct
    pub fn new() -> Result<Config, Box<dyn Error>> {
        if !config_exists(&FILES) {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Configuration file not found.",
            )));
        }

        let mut file = open_first_available_file(&FILES)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;
        let config: GlobalConfig = toml::from_str(&contents)?;

        Ok(Config {
            config,
        })
    }

    // Getter for the secret key
    pub fn get_secret_key(&self) -> &str {
        &self.config.session.secret_key
    }
}

// Lazy_static is used to initialize the configuration only once and share it across the application
lazy_static! {
    static ref CONFIG: Config = Config::new().unwrap();
}
