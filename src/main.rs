use rust_jpl::{Ephemeris, JulianDate};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config_path = "config.toml";

    println!("Initializing Ephemeris from {}...", config_path);
    let eph = match Ephemeris::new(config_path) {
        Ok(e) => e,
        Err(err) => {
            eprintln!("Error initializing ephemeris: {}", err);
            std::process::exit(1);
        }
    };

    // Display metadata
    let metadata = eph.get_metadata();
    println!("\nEphemeris Metadata:");
    println!(
        "  Date Range: {} - {} (Julian: {} - {})",
        metadata.start_year, metadata.end_year, metadata.julian_start, metadata.julian_end
    );
    println!("  Interval: {} days", metadata.interval_days);
    println!("  Earth-Moon Ratio: {}", metadata.earth_moon_ratio);
    println!(
        "  Number of Coefficients: {}",
        metadata.number_of_coefficients
    );

    // List available bodies
    println!("\nAvailable Celestial Bodies:");
    for body in eph.get_bodies() {
        println!("  {} (active: {})", body.name, body.active);
    }

    // Example: Get position for a specific date
    println!("\nExample: Getting planetary positions");
    let jd = JulianDate::from_calendar(2024, 1, 15, 12, 0, 0.0)?;
    println!("Julian Date: {}", jd.as_f64());

    let cal = jd.to_calendar();
    println!(
        "Calendar Date: {}-{:02}-{:02} {:02}:{:02}:{:02}",
        cal.year, cal.month, cal.day, cal.hour, cal.minute, cal.second as i32
    );

    // Try to get positions for various bodies
    let bodies_to_query = ["Sun", "Earth", "Moon", "Mars", "Jupiter"];
    for body_name in &bodies_to_query {
        match eph.get_position(body_name, jd) {
            Ok(pos) => {
                println!(
                    "  {}: Position ({:.6}, {:.6}, {:.6}) AU, Distance: {:.6} AU",
                    body_name,
                    pos.x,
                    pos.y,
                    pos.z,
                    pos.distance()
                );
            }
            Err(e) => {
                println!("  {}: Error - {}", body_name, e);
            }
        }
    }

    Ok(())
}
