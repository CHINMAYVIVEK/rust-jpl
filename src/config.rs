use dotenv::dotenv;
use std::env;
use std::fs;

#[derive(Debug)]
pub struct AppConfig {
    pub nasa_jpl_de441: String,
    pub header_441: String,
    pub initial_data_dat: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, String> {
        dotenv().ok(); // Load environment variables from a .env file if it exists

        Ok(Self {
            nasa_jpl_de441: Self::load_env_var("NASA_JPL_DE441", "assets/linux_m13000p17000.441.bsp")?,
            header_441: Self::load_env_var("HEADER_441", "assets/header.441")?,
            initial_data_dat: Self::load_env_var("INITIAL_DATA_DAT", "assets/Initial_data.dat")?,
        })
    }

    fn load_env_var(var_name: &str, default_value: &str) -> Result<String, String> {
        match env::var(var_name) {
            Ok(val) => {
                if let Err(_) = fs::metadata(&val) {
                    return Err(format!("Required file not found: {}", val));
                }
                Ok(val)
            }
            Err(_) => {
                if let Err(_) = fs::metadata(default_value) {
                    return Err(format!("Required file not found: {}", default_value));
                }
                Ok(default_value.to_string())
            }
        }
    }
}
