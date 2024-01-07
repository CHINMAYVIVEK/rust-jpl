
use std::io::{self};

mod app;

use app::{config, initialization};




fn main()  -> io::Result<()>  {
    // Load configuration
    let config = match config::conf::AppConfig::new() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("Error initializing configuration: {}", err);
            std::process::exit(1);
        }
    };

    // Use configuration values
    println!("NASA JPL DE441: {}", config.nasa_jpl_de441);
    println!("Header 441: {}", config.header_441);
    println!("Initial Data Dat: {}", config.initial_data_dat);

    let mut ephemerides_data = initialization::init::Ephemerides::default();

    initialization::init::initialization(&mut ephemerides_data)?;

    match config.read_nasa_jpl_de441() {
        Ok(contents) => {
            // Work with the file contents as needed
            println!("Read {} bytes from NASA JPL DE441 file. \n", contents.len());
            println!("Contents of the NASA JPL Ephemeris file:\n{:#?}", contents);
        }
        Err(err) => eprintln!("Error reading NASA JPL DE441 file: {}", err),
    }

    Ok(())
}
