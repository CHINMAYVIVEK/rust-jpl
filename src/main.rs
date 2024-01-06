
use std::fs::File;
use std::io::{self, Read};

mod config;
use config::AppConfig;

fn main()  -> io::Result<()>  {
    // Load configuration
    let config = match AppConfig::new() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("Error initializing configuration: {}", err);
            std::process::exit(1);
        }
    };

    // Use configuration values
    println!("NASA JPL DE441: {}", config.nasa_jpl_de441);

    // Attempt to open the file
    let mut file = File::open(&config.nasa_jpl_de441).expect("open");

    // Read the contents of the file
    let mut mutbuf = vec![0; 0];
    file.read_to_end(&mut mutbuf).expect("to end");
    let buffer = mutbuf.clone();

    // Print the contents of the file
    println!("Contents of the NASA JPL Ephemeris file:\n{:#?}", buffer);

    println!("Header 441: {}", config.header_441);
    println!("Initial Data Dat: {}", config.initial_data_dat);

    Ok(())
}
