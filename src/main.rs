
use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Read the path to the NASA JPL Ephemeris file from the environment
    let de404_path = env::var("NASA_JPL_DE441").expect("NASA_JPL_DE441 not set in .env");

    // Attempt to open the file
    let mut file = File::open(&de404_path).expect("open");

    // Read the contents of the file
    let mut mutbuf = vec![0; 0];
    file.read_to_end(&mut mutbuf).expect("to end");
    let buffer = mutbuf.clone();

    // Print the contents of the file
    println!("Contents of the NASA JPL Ephemeris file:\n{:#?}", buffer);

    Ok(())
}
