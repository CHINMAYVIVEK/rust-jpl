//! Basic usage example for rust-jpl
//!
//! This example demonstrates how to:
//! - Initialize an ephemeris
//! - Convert between calendar dates and Julian dates
//! - Get planetary positions
//! - Access ephemeris metadata

use rust_jpl::Ephemeris;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rust JPL Ephemeris Reader - Basic Usage Example ===\n");

    // Initialize the ephemeris from config.toml
    println!("1. Initializing Ephemeris...");
    let eph = Ephemeris::new("config.toml")?;
    println!("   ✓ Ephemeris loaded successfully\n");

    // Display metadata
    println!("2. Ephemeris Metadata:");
    let metadata = eph.get_metadata();
    println!(
        "   - Date Range: {} - {}",
        metadata.start_year, metadata.end_year
    );
    println!(
        "   - Julian Date Range: {:.2} - {:.2}",
        metadata.julian_start, metadata.julian_end
    );
    println!("   - Interval: {} days", metadata.interval_days);
    println!(
        "   - Earth-Moon Mass Ratio: {:.6}",
        metadata.earth_moon_ratio
    );
    println!(
        "   - Number of Coefficients: {}\n",
        metadata.number_of_coefficients
    );

    // List available bodies
    println!("3. Available Celestial Bodies:");
    for body in eph.get_bodies() {
        let status = if body.active { "✓" } else { "✗" };
        println!(
            "   {} {} ({})",
            status,
            body.name,
            if body.active { "active" } else { "inactive" }
        );
    }
    println!();

    // Time conversion example
    println!("4. Time Conversion Example:");
    let calendar_date = rust_jpl::CalendarDate::new(2024, 1, 15, 12, 0, 0.0);
    println!(
        "   Calendar Date: {}-{:02}-{:02} {:02}:{:02}:{:02}",
        calendar_date.year,
        calendar_date.month,
        calendar_date.day,
        calendar_date.hour,
        calendar_date.minute,
        calendar_date.second as i32
    );

    let jd = calendar_date.to_julian()?;
    println!("   Julian Date: {:.6}", jd.as_f64());

    let converted_back = jd.to_calendar();
    println!(
        "   Converted back: {}-{:02}-{:02} {:02}:{:02}:{:02}\n",
        converted_back.year,
        converted_back.month,
        converted_back.day,
        converted_back.hour,
        converted_back.minute,
        converted_back.second as i32
    );

    // Get planetary positions
    println!(
        "5. Planetary Positions (at Julian Date {:.6}):",
        jd.as_f64()
    );
    let bodies = [
        "Sun", "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn",
    ];

    for body_name in &bodies {
        match eph.get_position(body_name, jd) {
            Ok(pos) => {
                println!(
                    "   {}: ({:12.6}, {:12.6}, {:12.6}) AU, Distance: {:.6} AU",
                    body_name,
                    pos.x,
                    pos.y,
                    pos.z,
                    pos.distance()
                );
            }
            Err(e) => {
                println!("   {}: Error - {}", body_name, e);
            }
        }
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
