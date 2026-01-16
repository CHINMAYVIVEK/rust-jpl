//! Planetary positions example
//!
//! Demonstrates how to query planetary positions for multiple dates

use rust_jpl::{Ephemeris, JulianDate};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Planetary Positions Example ===\n");

    // Initialize ephemeris
    let eph = Ephemeris::new("config.toml")?;

    // Define dates to query
    let dates = vec![
        (2024, 1, 1, 0, 0, 0.0),
        (2024, 6, 15, 12, 0, 0.0),
        (2024, 12, 31, 23, 59, 59.0),
    ];

    let bodies = vec!["Sun", "Mercury", "Venus", "Earth", "Mars", "Jupiter"];

    for (year, month, day, hour, minute, second) in dates {
        let jd = JulianDate::from_calendar(year, month, day, hour, minute, second)?;
        println!(
            "Date: {}-{:02}-{:02} {:02}:{:02}:{:02} (JD: {:.6})",
            year,
            month,
            day,
            hour,
            minute,
            second as i32,
            jd.as_f64()
        );
        println!("{}", "=".repeat(70));

        for body_name in &bodies {
            match eph.get_position(body_name, jd) {
                Ok(pos) => {
                    println!(
                        "  {:10} | X: {:12.6} | Y: {:12.6} | Z: {:12.6} | Distance: {:10.6} AU",
                        body_name,
                        pos.x,
                        pos.y,
                        pos.z,
                        pos.distance()
                    );
                }
                Err(e) => {
                    println!("  {:10} | Error: {}", body_name, e);
                }
            }
        }
        println!();
    }

    Ok(())
}
