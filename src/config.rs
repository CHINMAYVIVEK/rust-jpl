//! Configuration management for ephemeris data

use config::{Config, File};
use serde::Deserialize;
use std::fs;
use std::fs::File as StdFile;
use std::io::{self, Read};

use crate::Error;

#[derive(Debug, Deserialize)]
struct ConfigPaths {
    nasa_jpl_de441: String,
    header_441: String,
    initial_data_dat: String,
}

#[derive(Debug, Deserialize)]
struct ConfigFile {
    paths: ConfigPaths,
}

/// Configuration for the ephemeris system
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub nasa_jpl_de441: String,
    pub header_441: String,
    pub initial_data_dat: String,
}

impl AppConfig {
    /// Create a new configuration from a config file
    ///
    /// # Arguments
    /// * `config_path` - Path to the config.toml file
    pub fn new(config_path: &str) -> Result<Self, Error> {
        let config = Config::builder()
            .add_source(File::with_name(config_path))
            .build()
            .map_err(|e| Error::Config(format!("Failed to load {}: {}", config_path, e)))?;

        let config_file: ConfigFile = config
            .try_deserialize()
            .map_err(|e| Error::Config(format!("Failed to deserialize config: {}", e)))?;

        let nasa_jpl_de441 = config_file.paths.nasa_jpl_de441.clone();
        let header_441 = config_file.paths.header_441.clone();
        let initial_data_dat = config_file.paths.initial_data_dat.clone();

        Self::validate_file(&nasa_jpl_de441)?;
        Self::validate_file(&header_441)?;
        Self::validate_file(&initial_data_dat)?;

        Ok(Self {
            nasa_jpl_de441,
            header_441,
            initial_data_dat,
        })
    }

    fn validate_file(file_path: &str) -> Result<(), Error> {
        fs::metadata(file_path)
            .map_err(|_| Error::Config(format!("Required file not found: {}", file_path)))?;
        Ok(())
    }

    /// Read the NASA JPL DE441 binary file
    pub fn read_nasa_jpl_de441(&self) -> Result<Vec<u8>, Error> {
        read_file(&self.nasa_jpl_de441).map_err(Error::from)
    }

    /// Read the header file
    pub fn read_header_441(&self) -> Result<Vec<u8>, Error> {
        read_file(&self.header_441).map_err(Error::from)
    }

    /// Read the initial data file
    pub fn read_initial_data_dat(&self) -> Result<Vec<u8>, Error> {
        read_file(&self.initial_data_dat).map_err(Error::from)
    }
}

fn read_file(file_path: &str) -> Result<Vec<u8>, io::Error> {
    let mut file = StdFile::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
